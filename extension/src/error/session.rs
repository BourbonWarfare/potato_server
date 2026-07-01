use crate::error::types::{KindId, TopicId};
use crate::error::{ErrorClassId, IdentifiableError, identifiable_error};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Topic {
    SessionError = 1,
}

#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SessionError {
    #[error("There is no current session")]
    NoCurrentSession = 1,
    #[error("Insufficient playercount")]
    InsufficientPlayercount = 2,
    #[error("No mission exists with provided information")]
    NoMissionExists = 3,
    #[error("Could not get session for unknown reason")]
    CouldNotGetSession = 4,
    #[error("Could not end safe start for unknown reason")]
    CouldNotEndSafeStart = 5,
    #[error("Could not end mission for unknown reason")]
    CouldNotEndMission = 6,
}

impl From<SessionError> for KindId {
    fn from(value: SessionError) -> Self {
        KindId::new(value as u8)
    }
}

identifiable_error!(SessionError, ErrorClassId::Session, Topic::SessionError);
