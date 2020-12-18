use log::{debug, info};
use nanoid::nanoid;
use sled::Db;
use tokio::sync::{
    mpsc::{self},
    oneshot,
};

trait Responder<R> {
    fn respond() -> R;
}

#[derive(Debug)]
pub enum Command<T: Persist> {
    Get {
        key: String,
        responder: oneshot::Sender<Option<T>>,
    },
    Persist {
        value: T,
        responder: oneshot::Sender<Result<bool, String>>,
    },
    Remove {
        key: String,
        responder: oneshot::Sender<Result<bool, String>>,
    },
    Count {
        responder: oneshot::Sender<usize>,
    },
    Purge {
        responder: oneshot::Sender<Result<usize, sled::Error>>,
    },
}

use crate::model::game::Game;

use super::Persist;

pub struct AsyncRepository<T: Persist> {
    path: String,
    db: Db,
    receiver: mpsc::Receiver<Command<T>>,
    sender: mpsc::Sender<Command<T>>,
}

impl<T: Persist> AsyncRepository<T> {
    pub fn init(path: &str) -> AsyncRepository<T> {
        let db = sled::open(if cfg!(test) {
            format!(".sled/{}/{}", nanoid!(), &path)
        } else {
            format!(".sled/{}", &path)
        })
        .expect("opening database has failed");

        let (tx, rx): (mpsc::Sender<Command<T>>, mpsc::Receiver<Command<T>>) = mpsc::channel(32);

        let repo = AsyncRepository {
            db,
            path: String::from(path),
            receiver: rx,
            sender: tx,
        };

        repo.purge_data()
            .expect("Cleanup of existing database has failed");

        repo
    }

    pub fn sender(&self) -> mpsc::Sender<Command<T>> {
        self.sender.clone()
    }

    pub async fn start_listening(&mut self) {
        info!("Database for \"{}\" ready", self.path);

        while let Some(cmd) = self.receiver.recv().await {
            debug!("Received message");

            match cmd {
                Command::Get { key, responder } => {
                    let _ = responder.send(self.find_by_id(&key));
                }
                Command::Persist { value, responder } => {
                    let _ = responder.send(self.persist(&value));
                }
                Command::Remove { key, responder } => {
                    let _ = responder.send(self.remove(&key));
                }
                Command::Count { responder } => {
                    let _ = responder.send(self.total_count());
                }
                Command::Purge { responder } => {
                    let _ = responder.send(self.purge_data());
                }
            }
        }
    }

    fn persist(&self, elem: &T) -> Result<bool, String> {
        self.db
            .insert(elem.id(), elem.clone())
            .expect("Persisting item failed");
        self.flush().map_err(|e| e.to_string()).map(|_| true)
    }

    fn remove(&self, id: &str) -> Result<bool, String> {
        self.db.remove(id).expect("Removing item failed");
        self.flush().map_err(|e| e.to_string()).map(|_| true)
    }

    fn find_by_id(&self, id: &str) -> Option<T> {
        let success = self.db.get(id);
        match success {
            Ok(res) => res.and_then(|g| T::try_from(g).ok()),
            Err(_) => None,
        }
    }

    fn total_count(&self) -> usize {
        self.db.len()
    }

    fn flush(&self) -> Result<bool, sled::Error> {
        self.db.flush().map(|_| true)
    }

    fn purge_data(&self) -> Result<usize, sled::Error> {
        let res = self.db.clear().and_then(|()| self.db.flush());
        info!("Purged database \"{}\"", self.path);

        res
    }

    pub async fn find_by_id_async(id: &str, sender: &mpsc::Sender<Command<T>>) -> Option<T> {
        AsyncRepository::run_query(
            |tx| Command::Get {
                key: String::from(id),
                responder: tx,
            },
            sender,
        )
        .await
    }

    pub async fn persist_async(
        elem: &T,
        sender: &mpsc::Sender<Command<T>>,
    ) -> Result<bool, String> {
        AsyncRepository::run_query(
            |tx| Command::Persist {
                value: elem.clone(),
                responder: tx,
            },
            sender,
        )
        .await
    }

    pub async fn remove_async(
        key: &str,
        sender: &mpsc::Sender<Command<T>>,
    ) -> Result<bool, String> {
        AsyncRepository::run_query(
            |tx| Command::Remove {
                key: String::from(key),
                responder: tx,
            },
            sender,
        )
        .await
    }

    pub async fn total_count_async(sender: &mpsc::Sender<Command<T>>) -> usize {
        AsyncRepository::run_query(|tx| Command::Count { responder: tx }, sender).await
    }

    pub async fn purge_data_async(sender: &mpsc::Sender<Command<T>>) -> Result<usize, sled::Error> {
        AsyncRepository::run_query(|tx| Command::Purge { responder: tx }, sender).await
    }

    async fn run_query<R>(
        cmd_provider: impl Fn(oneshot::Sender<R>) -> Command<T>,
        sender: &mpsc::Sender<Command<T>>,
    ) -> R {
        let (tx, rx): (oneshot::Sender<R>, oneshot::Receiver<R>) = oneshot::channel();
        let cmd = cmd_provider(tx);
        let _ = sender.clone().send(cmd).await;

        let res = rx.await;
        res.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game::Game;
    use crate::persistence::Persist;
    use nanoid::nanoid;
    use tokio::sync::mpsc;

    use super::{AsyncRepository, Command};

    fn init_repo() -> mpsc::Sender<Command<Game>> {
        let mut repo = AsyncRepository::init("games");

        let sender = repo.sender();
        tokio::spawn(async move {
            repo.start_listening().await;
        });

        sender
    }

    #[tokio::test]
    async fn should_find_game() {
        let s = init_repo();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());

        AsyncRepository::persist_async(&game, &s)
            .await
            .expect("Game persist failed");

        let res = AsyncRepository::find_by_id_async(&game_id, &s).await;

        assert!(res.is_some());
        assert_eq!(res.unwrap().id(), game_id);
    }

    #[tokio::test]
    async fn should_persist_game() {
        let s = init_repo();

        AsyncRepository::persist_async(&Game::new("admin", "token"), &s)
            .await
            .expect("Game persist failed");
    }

    #[tokio::test]
    async fn should_remove_game() {
        let s = init_repo();
        let game = Game::new("admin", "token");
        AsyncRepository::persist_async(&game, &s)
            .await
            .expect("Game persist failed");

        let persisted_game = AsyncRepository::find_by_id_async(&game.id(), &s).await;
        assert!(persisted_game.is_some());

        let res = AsyncRepository::remove_async(game.id(), &s)
            .await
            .expect("Removing game failed");
        assert!(res);

        let removed_game = AsyncRepository::find_by_id_async(&game.id(), &s).await;
        assert!(removed_game.is_none());
    }

    #[tokio::test]
    async fn should_not_find_game() {
        let s = init_repo();
        let res = AsyncRepository::find_by_id_async("unknown", &s).await;

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn should_purge_games() {
        let s = init_repo();
        AsyncRepository::persist_async(&Game::new("admin", "token"), &s)
            .await
            .expect("Game persist failed");

        assert_eq!(AsyncRepository::total_count_async(&s).await, 1);

        AsyncRepository::purge_data_async(&s)
            .await
            .expect("Cleanup has failed");

        assert_eq!(AsyncRepository::total_count_async(&s).await, 0);
    }

    #[tokio::test]
    async fn should_create_entities_concurrently() {
        let s = init_repo();

        let mut threads = vec![];

        for _ in 0..1000 {
            threads.push(async {
                let _ = AsyncRepository::persist_async(&Game::new("admin", &nanoid!()), &s).await;
            });
        }

        futures::future::join_all(threads).await;

        assert_eq!(AsyncRepository::total_count_async(&s).await, 1000);
    }
}
