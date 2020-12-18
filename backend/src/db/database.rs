use log::{debug, info};
use nanoid::nanoid;
use sled::Db;
use tokio::sync::mpsc::{self};

use super::{Command, Persist};

pub struct Database<T: Persist> {
    path: String,
    db: Db,
    receiver: mpsc::Receiver<Command<T>>,
    sender: mpsc::Sender<Command<T>>,
}

impl<T: Persist> Database<T> {
    pub fn init(path: &str) -> Database<T> {
        let db = sled::open(if cfg!(test) {
            format!(".sled/{}/{}", nanoid!(), &path)
        } else {
            format!(".sled/{}", &path)
        })
        .expect("opening database has failed");

        let (tx, rx): (mpsc::Sender<Command<T>>, mpsc::Receiver<Command<T>>) = mpsc::channel(32);

        let repo = Database {
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
            debug!("Received query: {:?}", cmd);

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
}
