use crate::{
    model::{Player, Task, TaskType},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VoteTask {
    pub id: String,
    pub option: String,
}

#[async_trait]
impl Task for VoteTask {
    fn get_type(&self) -> TaskType {
        TaskType::Vote
    }

    async fn apply_result(&self, player: Player, ctx: &AppContext) -> Result<(), String> {
        match ctx.db().votings().get(&self.id).await {
            Ok(voting) => match voting {
                Some(mut voting) => {
                    voting.add_vote(player.id(), "yes");
                    ctx.db()
                        .votings()
                        .persist(&voting)
                        .await
                        .expect("Writing voting has failed");

                    Ok(())
                }
                None => Err(String::from("Voting not found")),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    fn resolve_after_first_answer(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{Player, TaskDefinition, VoteOption, Voting},
        server::{app_context::AppContext, auth::generate_jwt_token, endpoints::tasks::apply_task},
    };
    use warp::{hyper::StatusCode, Reply};

    use super::VoteTask;

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_vote_yes() {
        let ctx = init_ctx();

        let mut player = Player::new("GAME");
        let voting = Voting::new(
            "test",
            &vec![String::from(player.id())],
            &vec![VoteOption::new("yes", "Yes"), VoteOption::new("no", "No")],
        );
        let voting_id = String::from(voting.id());
        ctx.db()
            .votings()
            .persist(&voting)
            .await
            .expect("Writing voting has failed");

        player.assign_task(TaskDefinition::Vote { voting });
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            VoteTask {
                id: voting_id.clone(),
                option: String::from("yes"),
            },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);

        assert_eq!(
            ctx.db()
                .votings()
                .get(&voting_id)
                .await
                .expect("Reading voting has failed")
                .unwrap()
                .votes()
                .get(player.id())
                .unwrap(),
            "yes"
        );
    }
}
