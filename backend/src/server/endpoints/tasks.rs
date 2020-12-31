use crate::{
    model::Task,
    server::{
        app_context::AppContext,
        auth::extract_verified_id,
        reply::{reply_error, reply_error_with_details, reply_success},
    },
};
use log::warn;
use std::convert::Infallible;
use warp::hyper::StatusCode;

pub async fn apply_task<T: Task>(
    task: T,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match extract_verified_id(authorization, ctx) {
        Some(player_id) => match ctx
            .db()
            .players()
            .get(&player_id)
            .await
            .expect("Reading player has failed")
        {
            Some(player) => {
                // Check if task is assigned
                if player
                    .open_tasks()
                    .front()
                    .filter(|def| def.get_type() == task.get_type())
                    .is_none()
                {
                    // Prevent leaking information about assigned tasks of other players by sending still OK
                    warn!(
                        "Player {} doesn't have task {:?} to resolve",
                        player.id(),
                        task.get_type()
                    );
                    return Ok(reply_success(StatusCode::OK));
                }
                let player_id = player.id().to_owned();
                match task.apply_result(player, ctx).await {
                    Ok(_) => {
                        if task.resolve_after_first_answer() {
                            let mut player = ctx
                                .db()
                                .players()
                                .get(&player_id)
                                .await
                                .expect("Loading player has failed")
                                .unwrap();
                            player.resolve_task(task.get_type());
                            if ctx.db().players().persist(&player).await.is_err() {
                                return Ok(reply_error_with_details(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Updating player has failed",
                                ));
                            }
                        }

                        Ok(reply_success(StatusCode::OK))
                    }
                    Err(err) => Ok(reply_error_with_details(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &err,
                    )),
                }
            }
            None => Ok(reply_error(StatusCode::UNAUTHORIZED)),
        },
        None => Ok(reply_error(StatusCode::UNAUTHORIZED)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::Player,
        server::{
            app_context::AppContext, auth::generate_jwt_token, endpoints::tasks::apply_task,
            tasks::settings::SettingsTask,
        },
    };
    use warp::{hyper::StatusCode, Reply};

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_do_nothing_for_unassigned_tasks() {
        let ctx = init_ctx();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            SettingsTask {
                name: String::from("Test"),
            },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);

        let updated_player = ctx
            .db()
            .players()
            .get(player.id())
            .await
            .expect("Reading player has failed")
            .unwrap();
        assert_eq!(updated_player.name(), player.name());
    }
}
