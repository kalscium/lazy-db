use crate::*;
use std::path::Path;

pub trait LazyObject {
    fn as_container(&self) -> LazyContainer;
    fn store_lazy(path: impl AsRef<Path>) -> Result<(), LDBError>;
    fn load_lazy(data: LazyContainer) -> Result<(), LDBError>;
}