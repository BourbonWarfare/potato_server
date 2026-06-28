use arma_rs::{Extension, arma};

mod backend;
mod error;
mod http;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group("backend", backend::group())
        .finish()
}
