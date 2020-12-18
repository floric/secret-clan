mod client;
mod database;

use sled::IVec;
use std::{clone::Clone, convert::TryFrom, fmt::Debug};
use tokio::sync::oneshot;

pub use self::client::Client;
pub use self::database::Database;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone + Debug + Send {
    fn id(&self) -> &str;
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
}
