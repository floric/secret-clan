use std::convert::Infallible;

use crate::server::{app_context::AppContext, auth::verify_jwt_token, errors::reply_with_error};
use serde::{Deserialize, Serialize};
use warp::hyper::StatusCode;

use super::games::{get_player_by_id, persist_player};

// GET /api/players/:id
pub async fn get_player_filter(id: &str, ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct GetPlayerResponse {
        id: String,
        name: String,
    }

    match get_player_by_id(ctx, &id).await {
        Some(player) => Ok(warp::reply::with_status(
            warp::reply::json(&GetPlayerResponse {
                id: String::from(player.id()),
                name: String::from(player.name()),
            }),
            StatusCode::OK,
        )),
        None => Ok(reply_with_error(StatusCode::NOT_FOUND)),
    }
}

#[derive(Deserialize)]
pub struct EditPlayerInput {
    name: String,
}

// POST /api/players/:id
pub async fn edit_player_filter(
    id: &str,
    input: &EditPlayerInput,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match verify_jwt_token(&authorization, &ctx.config().auth_secret)
        .ok()
        .and_then(|token| {
            token
                .claims()
                .get("sub")
                .map(String::from)
                .filter(|token_id| token_id == id)
        }) {
        Some(player_id) => match get_player_by_id(ctx, &player_id).await {
            Some(mut player) => {
                player.set_name(&input.name);
                persist_player(ctx, &player)
                    .await
                    .expect("editing player failed");
                Ok(warp::reply::with_status(
                    warp::reply::json(&player),
                    StatusCode::OK,
                ))
            }
            None => Ok(reply_with_error(StatusCode::UNAUTHORIZED)),
        },
        None => Ok(reply_with_error(StatusCode::UNAUTHORIZED)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::player::Player,
        server::{
            app_context::AppContext,
            auth::generate_jwt_token,
            endpoints::games::{get_player_by_id, persist_player},
        },
    };
    use warp::{hyper::StatusCode, Reply};

    use super::{edit_player_filter, get_player_filter, EditPlayerInput};

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
        persist_player(&ctx, &player)
            .await
            .expect("Writing player failed");

        let reply = get_player_filter(&player_id, &ctx).await;

        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_edit_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        let player_id = String::from(player.id());
        persist_player(&ctx, &player)
            .await
            .expect("Writing player failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = edit_player_filter(
            &player_id,
            &EditPlayerInput {
                name: String::from("new name"),
            },
            &token,
            &ctx,
        )
        .await;

        let updated_player = get_player_by_id(&ctx, player.id())
            .await
            .expect("Reading player failed");

        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
        assert_eq!(updated_player.name(), "new name");
    }

    #[tokio::test]
    async fn should_not_edit_other_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        persist_player(&ctx, &player)
            .await
            .expect("Writing player failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = edit_player_filter(
            "other",
            &EditPlayerInput {
                name: String::from("new name"),
            },
            &token,
            &ctx,
        )
        .await;

        let updated_player = get_player_by_id(&ctx, player.id())
            .await
            .expect("Reading player failed");

        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(updated_player.name(), player.name());
    }
}
