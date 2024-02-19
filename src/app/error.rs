use crate::domain::DomainError;
use crate::infra::InfraError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        todo!()
    }
}

impl From<InfraError> for AppError {
    fn from(value: InfraError) -> Self {
        todo!()
    }
}
