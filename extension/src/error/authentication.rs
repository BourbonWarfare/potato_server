use crate::error::types::{KindId, TopicId};
use crate::error::{ErrorClassId, IdentifiableError, identifiable_error};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Topic {
    LoginError = 1,
}

#[derive(Error, Clone, Debug, Eq, PartialEq, Hash)]
pub enum LoginError {
    #[error("The bot token provided is invalid")]
    InvalidBotToken,
    #[error("Could not login: {reason}")]
    CouldNotLogin { reason: String },
}
impl From<LoginError> for KindId {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::InvalidBotToken => KindId::new(1),
            LoginError::CouldNotLogin { .. } => KindId::new(2),
        }
    }
}
identifiable_error!(LoginError, ErrorClassId::Authentication, Topic::LoginError);
