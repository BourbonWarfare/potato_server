use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma, Debug, Clone, PartialEq, Eq)]
pub struct Status(pub u32);
