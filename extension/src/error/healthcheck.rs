use crate::error::types::{KindId, TopicId};
use crate::error::{ErrorClassId, IdentifiableError, identifiable_error};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Topic {
    HealthcheckError = 1,
}

#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum HealthcheckError {
    #[error("The healthcheck failed")]
    Failed = 1,
}
impl From<HealthcheckError> for KindId {
    fn from(value: HealthcheckError) -> Self {
        KindId::new(value as u8)
    }
}
identifiable_error!(
    HealthcheckError,
    ErrorClassId::Healthcheck,
    Topic::HealthcheckError
);
