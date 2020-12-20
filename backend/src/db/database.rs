use super::{Command, Persist};
use log::{debug, info, warn};
use nanoid::nanoid;
use sled::Db;
use std::collections::HashSet;
use tokio::sync::mpsc::{self};

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

        let (sender, receiver): (mpsc::Sender<Command<T>>, mpsc::Receiver<Command<T>>) =
            mpsc::channel(32);

        let repo = Database {
            db,
            path: String::from(path),
            receiver,
            sender,
        };

        repo.purge()
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
                Command::Get { key, data } => {
                    let _ = data.responder.send(self.get(&key));
                }
                Command::Persist { value, data } => {
                    let _ = data.responder.send(self.persist(&value));
                }
                Command::Remove { key, data } => {
                    let _ = data.responder.send(self.remove(&key));
                }
                Command::RemoveBatch { keys, data } => {
                    let _ = data.responder.send(self.remove_batch(&keys));
                }
                Command::Count { data } => {
                    let _ = data.responder.send(self.total_count());
                }
                Command::Scan {
                    scan_function,
                    data,
                } => {
                    let mut matching_ids = HashSet::new();
                    for x in self.db.iter() {
                        if let Ok((_, val)) = x {
                            let val = T::try_from(val).ok().unwrap();
                            let matches = scan_function(&val);
                            if matches {
                                matching_ids.insert(String::from(val.id()));
                            }
                        }
                    }
                    let _ = data.responder.send(Ok(matching_ids));
                }
                Command::Purge { data } => {
                    let _ = data.responder.send(self.purge());
                }
            }
        }
    }

    fn persist(&self, elem: &T) -> Result<bool, sled::Error> {
        self.db
            .insert(elem.id(), elem.clone())
            .expect("Persisting item failed");
        self.flush()
    }

    fn remove(&self, key: &str) -> Result<bool, sled::Error> {
        match self.db.remove(key).expect("Removing item failed") {
            Some(_) => self.flush(),
            None => {
                warn!("No item with key \"{}\" found for removal", key);
                Ok(false)
            }
        }
    }

    fn remove_batch(&self, keys: &HashSet<String>) -> Result<bool, sled::Error> {
        let batch = sled::Batch::default();
        for key in keys {
            match self.db.remove(key).expect("Removing item failed") {
                Some(_) => {}
                None => {
                    warn!("No item with key \"{}\" found for removal", key);
                }
            }
        }
        self.db.apply_batch(batch)?;
        self.flush()
    }

    fn get(&self, id: &str) -> Result<Option<T>, sled::Error> {
        let success = self.db.get(id);
        match success {
            Ok(res) => Ok(res.and_then(|g| T::try_from(g).ok())),
            Err(err) => Err(err),
        }
    }

    fn total_count(&self) -> usize {
        self.db.len()
    }

    fn flush(&self) -> Result<bool, sled::Error> {
        self.db.flush().map(|_| true)
    }

    fn purge(&self) -> Result<bool, sled::Error> {
        let res = self.db.clear().and_then(|()| self.db.flush()).map(|_| true);
        info!("Purged database \"{}\"", self.path);

        res
    }
}
