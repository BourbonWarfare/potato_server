use arma_rs::{IntoArma, Value};
use std::fmt::{self, Display, Formatter};

macro_rules! identifiable_error {
    ($ty:ty, $class:expr, $topic:expr) => {
        impl IdentifiableError for $ty {
            const CLASS: ErrorClassId = $class;
            const TOPIC: TopicId = TopicId::new($topic as u8);

            fn kind(self) -> KindId {
                KindId::new(self as u8)
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum ErrorClassId {
    Healthcheck = 100,
    Authentication = 110,
}

pub mod healthcheck {
    use super::{ErrorClassId, IdentifiableError, KindId, TopicId};
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
    identifiable_error!(
        HealthcheckError,
        ErrorClassId::Healthcheck,
        Topic::HealthcheckError
    );
}

pub mod authentication {
    use super::{ErrorClassId, IdentifiableError, KindId, TopicId};
    use thiserror::Error;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(u8)]
    enum Topic {
        LoginError = 1,
        SessionError = 2,
    }

    #[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(u8)]
    pub enum LoginError {
        #[error("The bot token provided is invalid")]
        InvalidBotToken = 1,
    }
    identifiable_error!(LoginError, ErrorClassId::Authentication, Topic::LoginError);

    #[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(u8)]
    pub enum SessionError {
        #[error("Session could not be refreshed")]
        CouldNotRefresh = 1,
    }
    identifiable_error!(
        SessionError,
        ErrorClassId::Authentication,
        Topic::SessionError
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct TopicId(u8);
impl TopicId {
    pub const fn new(id: u8) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct KindId(u8);
impl KindId {
    pub const fn new(id: u8) -> Self {
        Self(id)
    }
}

#[derive(IntoArma, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct ErrorId(u32);

impl ErrorId {
    pub const fn new(class: ErrorClassId, topic: TopicId, kind: KindId) -> Self {
        Self(((class as u32) << 16) | ((topic.0 as u32) << 8) | (kind.0 as u32))
    }
}

impl Display for ErrorId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#08X}", self.0)
    }
}

trait IdentifiableError: Display + Copy {
    const CLASS: ErrorClassId;
    const TOPIC: TopicId;

    fn kind(self) -> KindId;
    fn error_id(self) -> ErrorId {
        ErrorId::new(Self::CLASS, Self::TOPIC, self.kind())
    }
}

pub struct ArmaError<T>(pub T);
impl<T> From<T> for ArmaError<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> IntoArma for ArmaError<T>
where
    T: IdentifiableError,
{
    fn to_arma(&self) -> Value {
        Value::Array(vec![
            self.0.error_id().to_arma(),
            Value::String(self.0.to_string()),
        ])
    }
}
