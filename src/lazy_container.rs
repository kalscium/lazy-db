use std::rc::Rc;
use std::path::PathBuf;

pub struct LazyContainer {
    pub parent: Option<Rc<LazyContainer>>,
    pub is_stem: bool,
    pub path: PathBuf,
}