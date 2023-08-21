use crate::*;

#[macro_export]
macro_rules! cache_field {
    ($name:ident($this:ident, $logger:ident) -> $type:ty $code:block) => {
        #[allow(unused_mut)]
        pub fn $name(&mut self, mut $logger: impl Logger) -> &mut $type {
            let $this = self;
            if $this.$name.is_none() {
                $this.$name = Some($code);
            }; $this.$name.as_mut_ref().unwrap()
        }
    }
}

#[allow(drop_bounds)]
pub trait LazyObject: Drop {
    fn as_container(&self) -> &LazyContainer;
    fn store_lazy(&self) -> Result<(), LDBError>;
    fn load_lazy(container: LazyContainer) -> Self;
    fn clear_cache(&mut self);
}