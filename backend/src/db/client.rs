use super::{Command, CommandData, Persist, QueryError};
use log::debug;
use nanoid::nanoid;
use std::{
    collections::HashSet,
    fmt::{self, Debug},
    marker::PhantomData,
};
use tokio::sync::{mpsc, oneshot};
pub struct Client<T: Persist> {
    phantom: PhantomData<T>,
    sender: mpsc::Sender<Command<T>>,
}

impl<T: Persist> Client<T> {
    pub fn new(sender: mpsc::Sender<Command<T>>) -> Self {
        Client {
            phantom: PhantomData,
            sender,
        }
    }

    async fn run_query<R: Debug>(
        &self,
        cmd_provider: impl Fn(CommandData<R>) -> Command<T>,
    ) -> Result<R, QueryError> {
        let (responder, receiver): (oneshot::Sender<R>, oneshot::Receiver<R>) = oneshot::channel();
        let id = nanoid!();
        let data = CommandData {
            responder,
            id: String::from(&id),
        };
        let cmd = cmd_provider(data);
        let res = self.sender.clone().send(cmd).await;
        debug!("Sent query \"{}\"", &id);

        match res {
            Ok(_) => match receiver.await {
                Ok(res) => {
                    debug!("Received answer for query \"{}\": {:?}", &id, res);
                    Ok(res)
                }
                Err(error) => Err(QueryError::new(&fmt::format(format_args!(
                    "Retrieving result for query \"{}\" has failed: {}",
                    id,
                    error.to_string(),
                )))),
            },
            Err(error) => Err(QueryError::new(&fmt::format(format_args!(
                "Sending query for query \"{}\" has failed: {}",
                id,
                error.to_string(),
            )))),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Option<T>, QueryError> {
        self.run_query(|data| Command::Get {
            key: String::from(id),
            data,
        })
        .await
        .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn scan(&self, scan_function: fn(&T) -> bool) -> Result<HashSet<String>, QueryError> {
        self.run_query(|data| Command::Scan {
            scan_function,
            data,
        })
        .await
        .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn persist(&self, elem: &T) -> Result<bool, QueryError> {
        self.run_query(|data| Command::Persist {
            value: elem.clone(),
            data,
        })
        .await
        .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn remove(&self, key: &str) -> Result<bool, QueryError> {
        self.run_query(|data| Command::Remove {
            key: String::from(key),
            data,
        })
        .await
        .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn remove_batch(&self, keys: &HashSet<String>) -> Result<bool, QueryError> {
        self.run_query(|data| Command::RemoveBatch {
            keys: keys.clone(),
            data,
        })
        .await
        .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn purge(&self) -> Result<bool, QueryError> {
        self.run_query(|data| Command::Purge { data })
            .await
            .and_then(|x| x.map_err(QueryError::from_sled))
    }

    pub async fn total_count(&self) -> Result<usize, QueryError> {
        self.run_query(|data| Command::Count { data }).await
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::{
        db::{Database, Persist},
        model::game::Game,
    };
    use nanoid::nanoid;
    use std::collections::HashSet;

    fn init_client() -> Client<Game> {
        let mut repo = Database::init("games");

        let sender = repo.sender();
        tokio::task::spawn(async move {
            repo.start_listening().await;
        });

        let client = Client::new(sender);

        client
    }

    #[tokio::test]
    async fn should_get_game() {
        let client = init_client();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());

        client.persist(&game).await.expect("Game persist failed");

        let res = client.get(&game_id).await.expect("Reading game has failed");

        assert!(res.is_some());
        assert_eq!(res.unwrap().id(), game_id);
    }

    #[tokio::test]
    async fn should_persist_game() {
        let client = init_client();

        client
            .persist(&Game::new("admin", "token"))
            .await
            .expect("Game persist failed");
    }

    #[tokio::test]
    async fn should_remove_game() {
        let client = init_client();
        let game = Game::new("admin", "token");
        client.persist(&game).await.expect("Game persist failed");

        let persisted_game = client
            .get(&game.id())
            .await
            .expect("Reading game has failed");
        assert!(persisted_game.is_some());

        let res = client
            .remove(game.id())
            .await
            .expect("Removing game failed");
        assert!(res);

        let removed_game = client
            .get(&game.id())
            .await
            .expect("Reading game has failed");
        assert!(removed_game.is_none());
    }

    #[tokio::test]
    async fn should_remove_games() {
        let client = init_client();
        let mut ids = HashSet::new();
        for x in &["A", "B", "C"] {
            let g = Game::new("admin", *x);
            client.persist(&g).await.expect("Game persist failed");
            ids.insert(String::from(*x));
        }

        let game_count = client
            .total_count()
            .await
            .expect("Reading count has failed");
        assert_eq!(game_count, 3);

        let res = client
            .remove_batch(&ids)
            .await
            .expect("Removing games failed");
        assert!(res);

        let game_count = client
            .total_count()
            .await
            .expect("Reading count has failed");
        assert_eq!(game_count, 0);
    }

    #[tokio::test]
    async fn should_not_get_game() {
        let client = init_client();
        let res = client
            .get("unknown")
            .await
            .expect("Reading game has failed");

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn should_create_entities_concurrently() {
        let client = init_client();

        let mut threads = vec![];

        for _ in 0..1000 {
            threads.push(async {
                let _ = client.persist(&Game::new("admin", &nanoid!())).await;
            });
        }

        futures::future::join_all(threads).await;

        assert_eq!(
            client
                .total_count()
                .await
                .expect("Reading count has failed"),
            1000
        );
    }
}
