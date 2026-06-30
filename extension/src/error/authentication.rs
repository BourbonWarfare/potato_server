use crate::error::types::{KindId, TopicId};
use crate::error::{ErrorClassId, IdentifiableError, identifiable_error};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Topic {
    LoginError = 1,
}

#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum LoginError {
    #[error("The bot token provided is invalid")]
    InvalidBotToken = 1,
    #[error("Could not login")]
    CouldNotLogin = 2,
}
identifiable_error!(LoginError, ErrorClassId::Authentication, Topic::LoginError);
