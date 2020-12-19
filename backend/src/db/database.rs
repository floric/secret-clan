use log::info;
use nanoid::nanoid;
use sled::Db;
use std::collections::HashSet;
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

        let (sender, receiver): (mpsc::Sender<Command<T>>, mpsc::Receiver<Command<T>>) =
            mpsc::channel(32);

        let repo = Database {
            db,
            path: String::from(path),
            receiver,
            sender,
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
                Command::Scan {
                    scan_function,
                    responder,
                } => {
                    let mut matching_ids = HashSet::new();
                    for x in self.db.iter() {
                        match x {
                            Ok((_, val)) => {
                                let val = T::try_from(val).ok().unwrap();
                                let matches = scan_function(&val);
                                if matches {
                                    matching_ids.insert(String::from(val.id()));
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    let _ = responder.send(Ok(matching_ids));
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

    fn remove(&self, id: &str) -> Result<bool, sled::Error> {
        self.db.remove(id).expect("Removing item failed");
        self.flush()
    }

    fn find_by_id(&self, id: &str) -> Result<Option<T>, sled::Error> {
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

    fn purge_data(&self) -> Result<usize, sled::Error> {
        let res = self.db.clear().and_then(|()| self.db.flush());
        info!("Purged database \"{}\"", self.path);

        res
    }
}
