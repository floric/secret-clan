use super::{Command, Persist, ScanFunction};
use log::{debug, error, info, warn};
use nanoid::nanoid;
use rayon::prelude::*;
use sled::Db;
use std::collections::HashMap;
use tokio::sync::{
    mpsc::{self},
    oneshot::{self},
};

pub struct Database<T: Persist> {
    path: String,
    db: Db,
    receiver: mpsc::Receiver<Command<T>>,
}

impl<T: Persist> Database<T> {
    pub fn init(path: &str) -> (Database<T>, mpsc::Sender<Command<T>>) {
        let db = sled::open(if cfg!(test) {
            format!(".sled/{}/{}", nanoid!(), &path)
        } else {
            format!(".sled/{}", &path)
        })
        .expect("opening database has failed");

        let (sender, receiver): (mpsc::Sender<Command<T>>, mpsc::Receiver<Command<T>>) =
            mpsc::channel(256);

        let repo = Database {
            db,
            path: String::from(path),
            receiver,
        };

        repo.purge()
            .expect("Cleanup of existing database has failed");
        (repo, sender)
    }

    /// A database connection is etablished by creating a database instance, which should should then be started in a separate thread.
    /// The communication between resources and the database is established with channels. Simply use a client to send messages to the database thread in an easy accesible way.
    /// Of course it's also possible to send messages through the channel directly without using the client.
    ///
    /// Example:
    /// ```
    /// use secret_clan::{model::Game, db::{Database, Client, Command}};
    /// let (mut repo, sender): (Database<Game>, tokio::sync::mpsc::Sender<Command<Game>>) = Database::init("test");
    /// std::thread::spawn(move || {
    ///     repo.start_listening();
    /// });
    /// let client = Client::new(sender);
    /// ```
    pub async fn start_listening(&mut self) {
        info!("Database for \"{}\" ready", self.path);

        while let Some(cmd) = self.receiver.recv().await {
            debug!("Received query: {:?}", cmd);

            match cmd {
                Command::Get { key, data } => {
                    self.send_result(self.get(&key), data.responder);
                }
                Command::GetBatch { keys, data } => {
                    self.send_result(self.get_batch(&keys), data.responder);
                }
                Command::Persist { value, data } => {
                    self.send_result(self.persist(value), data.responder);
                }
                Command::PersistBatch { values, data } => {
                    self.send_result(self.persist_batch(&values), data.responder);
                }
                Command::Remove { key, data } => {
                    self.send_result(self.remove(&key), data.responder);
                }
                Command::RemoveBatch { keys, data } => {
                    self.send_result(self.remove_batch(&keys), data.responder);
                }
                Command::Count { data } => {
                    self.send_result(self.total_count(), data.responder);
                }
                Command::Scan {
                    scan_function,
                    data,
                } => {
                    self.send_result(Ok(self.scan(scan_function)), data.responder);
                }
                Command::Purge { data } => {
                    self.send_result(self.purge(), data.responder);
                }
            }
        }
    }

    fn persist(&self, elem: T) -> Result<(), sled::Error> {
        self.db
            .insert(elem.id(), elem.clone())
            .and_then(|_| self.flush())
    }

    fn persist_batch(&self, values: &[T]) -> Result<(), sled::Error> {
        let batch = sled::Batch::default();
        for elem in values {
            if self.db.insert(elem.id(), elem.clone()).is_err() {
                error!("Persisting item has failed");
            }
        }
        self.db.apply_batch(batch)?;
        self.flush()
    }

    fn remove(&self, key: &str) -> Result<(), sled::Error> {
        self.db.remove(key).and_then(|res| match res {
            Some(_) => self.flush(),
            None => {
                warn!("No item with key \"{}\" found for removal", key);
                Ok(())
            }
        })
    }

    fn remove_batch(&self, keys: &[String]) -> Result<(), sled::Error> {
        let batch = sled::Batch::default();
        for key in keys {
            if self.db.remove(key).is_err() {
                warn!("No item with key \"{}\" found for removal", key);
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

    fn get_batch(&self, ids: &[String]) -> Result<HashMap<String, T>, sled::Error> {
        let mut result = HashMap::new();

        for id in ids {
            if let Ok(val) = self.db.get(id) {
                if let Some(val) = val.and_then(|g| T::try_from(g).ok()) {
                    result.insert(String::from(id), val);
                }
            }
        }

        Ok(result)
    }

    fn scan(&self, scan_function: ScanFunction<T>) -> Vec<String> {
        self.db
            .iter()
            .par_bridge()
            .filter_map(Result::ok)
            .filter_map(|(_, x)| T::try_from(x).ok())
            .filter_map(|y| {
                if scan_function(&y) {
                    Some(String::from(y.id()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn total_count(&self) -> usize {
        self.db.len()
    }

    #[inline]
    fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush().map(|_| ())
    }

    #[inline]
    fn send_result<R>(&self, data: R, sender: oneshot::Sender<R>) {
        if sender.send(data).is_err() {
            error!("Sending result to client has failed");
        }
    }

    fn purge(&self) -> Result<(), sled::Error> {
        let res = self.db.clear().and_then(|()| self.db.flush()).map(|_| ());
        info!("Purged database \"{}\"", self.path);

        res
    }
}
