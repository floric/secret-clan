use super::{Command, CommandData, Persist, QueryError};
use log::debug;
use nanoid::nanoid;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
};
use tokio::sync::{mpsc, oneshot};

pub struct Client<T: Persist> {
    sender: mpsc::Sender<Command<T>>,
}

impl<T: Persist> Client<T> {
    pub fn new(sender: mpsc::Sender<Command<T>>) -> Self {
        Client { sender }
    }

    #[inline]
    async fn run_query<R: Debug>(
        &self,
        cmd_provider: impl FnOnce(CommandData<R>) -> Command<T>,
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

    #[inline]
    fn map_result<R>(res: Result<R, sled::Error>) -> Result<R, QueryError> {
        res.map_err(QueryError::from_sled)
    }

    pub async fn get(&self, id: &str) -> Result<Option<T>, QueryError> {
        let key = String::from(id);
        self.run_query(|data| Command::Get { key, data })
            .await
            .and_then(Self::map_result)
    }

    pub async fn get_batch(&self, ids: &[String]) -> Result<HashMap<String, T>, QueryError> {
        let keys = ids.iter().map(String::from).collect();
        self.run_query(|data| Command::GetBatch { keys, data })
            .await
            .and_then(Self::map_result)
    }

    pub async fn scan(
        &self,
        scan_function: Box<dyn Fn(&T) -> bool + Send + Sync>,
    ) -> Result<HashSet<String>, QueryError> {
        self.run_query(|data| Command::Scan {
            scan_function,
            data,
        })
        .await
        .and_then(Self::map_result)
    }

    pub async fn persist(&self, elem: &T) -> Result<(), QueryError> {
        self.run_query(|data| Command::Persist {
            value: elem.clone(),
            data,
        })
        .await
        .and_then(Self::map_result)
    }

    pub async fn persist_batch(&self, values: &[T]) -> Result<(), QueryError> {
        self.run_query(|data| Command::PersistBatch {
            values: values.to_owned(),
            data,
        })
        .await
        .and_then(Self::map_result)
    }

    pub async fn remove(&self, key: &str) -> Result<(), QueryError> {
        self.run_query(|data| Command::Remove {
            key: String::from(key),
            data,
        })
        .await
        .and_then(Self::map_result)
    }

    pub async fn remove_batch(&self, keys: &HashSet<String>) -> Result<(), QueryError> {
        self.run_query(|data| Command::RemoveBatch {
            keys: keys.clone(),
            data,
        })
        .await
        .and_then(Self::map_result)
    }

    pub async fn purge(&self) -> Result<(), QueryError> {
        self.run_query(|data| Command::Purge { data })
            .await
            .and_then(Self::map_result)
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
        model::Game,
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
    async fn should_get_games() {
        let client = init_client();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());

        client.persist(&game).await.expect("Game persist failed");

        let res = client
            .get_batch(&vec![game_id, String::from("unknown")])
            .await
            .expect("Reading game has failed");

        assert_eq!(res.len(), 1);
    }

    #[tokio::test]
    async fn should_persist_game() {
        let client = init_client();

        client
            .persist(&Game::new("admin", "token"))
            .await
            .expect("Game persist failed");
        assert_eq!(client.total_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn should_persist_games() {
        let client = init_client();

        let mut games = vec![];
        for i in 0..100 {
            games.push(Game::new(
                "admin",
                &std::fmt::format(format_args!("token{}", i)),
            ));
        }
        client
            .persist_batch(&games)
            .await
            .expect("Game persist failed");
        assert_eq!(client.total_count().await.unwrap(), 100);
    }

    #[tokio::test]
    async fn should_purge_games() {
        let client = init_client();

        client
            .persist(&Game::new("admin", "token"))
            .await
            .expect("Game persist failed");
        client
            .persist(&Game::new("admin", "token2"))
            .await
            .expect("Game persist failed");

        assert_eq!(client.total_count().await.unwrap(), 2);

        client.purge().await.expect("Perging has failed");

        assert_eq!(client.total_count().await.unwrap(), 0);
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
