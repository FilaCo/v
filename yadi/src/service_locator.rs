use crate::builder::ServiceLocatorBuilder;
use crate::Tag;
use std::sync::Arc;

pub struct ServiceLocator {}

impl ServiceLocator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn builder() -> ServiceLocatorBuilder {
        ServiceLocatorBuilder::default()
    }

    pub fn resolve<T>(&mut self, tag: Option<Tag>) -> Arc<&T> {
        todo!()
    }
}
