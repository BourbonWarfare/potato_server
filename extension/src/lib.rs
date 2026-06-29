use arma_rs::{Extension, arma};

mod backend;
mod error;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group("backend", backend::group())
        .finish()
}
