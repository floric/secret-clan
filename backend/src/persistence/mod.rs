mod repository;

use sled::IVec;
use std::{clone::Clone, convert::TryFrom};

pub use self::repository::Repository;

pub trait Persist: Into<IVec> + TryFrom<IVec> + Clone {
    fn id(&self) -> &str;
}
