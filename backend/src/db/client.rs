use std::{collections::HashSet, marker::PhantomData};

use log::error;
use tokio::sync::{mpsc, oneshot};

use super::{Command, Persist};

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

    async fn run_query<R>(
        &self,
        cmd_provider: impl Fn(oneshot::Sender<R>) -> Command<T>,
    ) -> Option<R> {
        let (tx, rx): (oneshot::Sender<R>, oneshot::Receiver<R>) = oneshot::channel();
        let cmd = cmd_provider(tx);
        let res = self.sender.clone().send(cmd).await;
        match res {
            Ok(_) => match rx.await {
                Ok(res) => Some(res),
                Err(error) => {
                    error!("Retrieving result has failed: {}", error.to_string());
                    None
                }
            },
            Err(error) => {
                error!("Sending Query has failed: {}", error.to_string());
                None
            }
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Option<T> {
        self.run_query(|responder| Command::Get {
            key: String::from(id),
            responder,
        })
        .await
        .expect("Reading query failed")
    }

    pub async fn scan(&self, scan_function: fn(&T) -> bool) -> HashSet<String> {
        self.run_query(|responder| Command::Scan {
            scan_function,
            responder,
        })
        .await
        .expect("Reading query failed")
    }

    pub async fn persist(&self, elem: &T) -> Result<bool, String> {
        self.run_query(|responder| Command::Persist {
            value: elem.clone(),
            responder,
        })
        .await
        .expect("Reading query failed")
    }

    pub async fn remove(&self, key: &str) -> Result<bool, String> {
        self.run_query(|responder| Command::Remove {
            key: String::from(key),
            responder,
        })
        .await
        .expect("Reading query failed")
    }

    pub async fn total_count(&self) -> usize {
        self.run_query(|responder| Command::Count { responder })
            .await
            .expect("Reading query failed")
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

    fn init_client() -> Client<Game> {
        let mut repo = Database::init("games");

        let sender = repo.sender();
        tokio::spawn(async move {
            repo.start_listening().await;
        });

        let client = Client::new(sender);

        client
    }

    #[tokio::test]
    async fn should_find_game() {
        let client = init_client();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());

        client.persist(&game).await.expect("Game persist failed");

        let res = client.find_by_id(&game_id).await;

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

        let persisted_game = client.find_by_id(&game.id()).await;
        assert!(persisted_game.is_some());

        let res = client
            .remove(game.id())
            .await
            .expect("Removing game failed");
        assert!(res);

        let removed_game = client.find_by_id(&game.id()).await;
        assert!(removed_game.is_none());
    }

    #[tokio::test]
    async fn should_not_find_game() {
        let client = init_client();
        let res = client.find_by_id("unknown").await;

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

        assert_eq!(client.total_count().await, 1000);
    }
}
