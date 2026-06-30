use arma_rs::IntoArma;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct TopicId(pub u8);
impl TopicId {
    pub const fn new(id: u8) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct KindId(pub u8);
impl KindId {
    pub const fn new(id: u8) -> Self {
        Self(id)
    }
}

#[derive(IntoArma, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ErrorId(pub u32);

impl Display for ErrorId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#08X}", self.0)
    }
}
