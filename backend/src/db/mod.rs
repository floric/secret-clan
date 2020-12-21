mod client;
mod database;

use sled::IVec;
use std::{
    clone::Clone,
    collections::HashSet,
    convert::TryFrom,
    fmt::{self, Debug},
};
use tokio::sync::oneshot;

pub use self::client::Client;
pub use self::database::Database;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone + Debug + Send {
    fn id(&self) -> &str;
}
#[derive(Debug, Clone)]
pub struct QueryError {
    message: String,
}

impl QueryError {
    pub fn new(message: &str) -> Self {
        QueryError {
            message: String::from(message),
        }
    }

    pub fn from_sled(error: sled::Error) -> Self {
        QueryError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct CommandData<R: Debug> {
    id: String,
    #[derivative(Debug = "ignore")]
    responder: oneshot::Sender<R>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Command<T: Persist> {
    Get {
        key: String,
        data: CommandData<Result<Option<T>, sled::Error>>,
    },
    Scan {
        #[derivative(Debug = "ignore")]
        scan_function: Box<dyn Fn(&T) -> bool + Send + Sync>,
        data: CommandData<Result<HashSet<String>, sled::Error>>,
    },
    Persist {
        value: T,
        data: CommandData<Result<bool, sled::Error>>,
    },
    Remove {
        key: String,
        data: CommandData<Result<bool, sled::Error>>,
    },
    Purge {
        data: CommandData<Result<bool, sled::Error>>,
    },
    RemoveBatch {
        keys: HashSet<String>,
        data: CommandData<Result<bool, sled::Error>>,
    },
    Count {
        data: CommandData<usize>,
    },
}
