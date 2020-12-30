use crate::server::{app_context::AppContext, reply::reply_error};
use serde::Serialize;
use std::convert::Infallible;
use warp::hyper::StatusCode;

pub async fn get_player_filter(id: &str, ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct GetPlayerResponse {
        id: String,
        name: String,
    }

    match ctx
        .db()
        .players()
        .get(&id)
        .await
        .expect("Reading player has failed")
    {
        Some(player) => Ok(warp::reply::with_status(
            warp::reply::json(&GetPlayerResponse {
                id: String::from(player.id()),
                name: String::from(player.name()),
            }),
            StatusCode::OK,
        )),
        None => Ok(reply_error(StatusCode::NOT_FOUND)),
    }
}

#[cfg(test)]
mod tests {
    use super::get_player_filter;
    use crate::{model::Player, server::app_context::AppContext};
    use warp::{hyper::StatusCode, Reply};

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_not_get_unknown_player() {
        let ctx = init_ctx();

        let reply = get_player_filter("unknown", &ctx).await;

        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_get_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        let player_id = String::from(player.id());
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Writing player failed");

        let reply = get_player_filter(&player_id, &ctx).await;

        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
    }
}
