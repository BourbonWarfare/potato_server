use arma_rs::{Extension, arma};

mod backend;
mod error;
mod orbat;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group("backend", backend::group())
        .finish()
}
