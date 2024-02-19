use crate::{Factory, ServiceLocator, Tag, YadiResult};

pub struct ServiceLocatorBuilder {}

impl ServiceLocatorBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_singleton<T>(&mut self, factory: Factory<T>, tag: Option<Tag>) -> &mut Self {
        self
    }

    pub fn add_transient<T>(&mut self, factory: Factory<T>, tag: Option<Tag>) -> &mut Self {
        self
    }

    pub fn build(self) -> YadiResult<ServiceLocator> {
        Ok(ServiceLocator::new())
    }
}

impl Default for ServiceLocatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
