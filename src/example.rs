use crate::*;
struct Person {
    container: LazyContainer,
    name: Option<String>,
    age: Option<u8>,
}

impl LazyObject for Person {
    fn as_container(&self) -> &LazyContainer {
        &self.container
    }

    fn store_lazy(&self) -> Result<(), LDBError> {
        if let Some(x) = &self.name {
            LazyData::new_string(self.container.data_writer("name")?, x)?;
        };

        if let Some(x) = self.age {
            LazyData::new_u8(self.container.data_writer("name")?, x)?
        };

        Ok(())
    }

    fn load_lazy(container: LazyContainer) -> Self {
        Self {
            container,
            name: None,
            age: None,
        }
    }

    fn clear_cache(&mut self) {
        self.name = None;
        self.age = None;
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        let _ = self.store_lazy();
    }
}