use crate::ServiceLocator;
use std::sync::Arc;

pub type Factory<T> = fn(&mut ServiceLocator) -> Arc<T>;
