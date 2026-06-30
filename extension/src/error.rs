use arma_rs::{IntoArma, Value};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum ErrorClassId {
    Healthcheck = 100,
    Authentication = 110,
    Session = 120,
}

pub mod authentication;
pub mod healthcheck;
pub mod session;
mod types;

impl types::ErrorId {
    pub const fn new(class: ErrorClassId, topic: types::TopicId, kind: types::KindId) -> Self {
        Self(((class as u32) << 16) | ((topic.0 as u32) << 8) | (kind.0 as u32))
    }
}

trait IdentifiableError: Display + Copy {
    const CLASS: ErrorClassId;
    const TOPIC: types::TopicId;

    fn kind(self) -> types::KindId;
    fn error_id(self) -> types::ErrorId {
        types::ErrorId::new(Self::CLASS, Self::TOPIC, self.kind())
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

#[macro_export]
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
pub use identifiable_error;
