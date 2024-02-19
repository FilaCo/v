use crate::Aggregate;
use async_trait::async_trait;

pub trait Repo<T: Aggregate, Q>: ReadRepo<T, Q> {}

#[async_trait]
pub trait ReadRepo<T: Aggregate, Q> {
    type Error;

    async fn get(&self, query: Q) -> Result<&[T], Self::Error>;
}
