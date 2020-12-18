mod async_repo;
mod repository;

use sled::IVec;
use std::{clone::Clone, convert::TryFrom};

pub use self::async_repo::{AsyncRepository, Command};
pub use self::repository::Repository;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone {
    fn id(&self) -> &str;
}
