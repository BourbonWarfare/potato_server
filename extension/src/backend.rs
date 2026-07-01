use arma_rs::Group;

mod authentication;
mod interface;
mod session;
mod status;

pub fn group() -> Group {
    Group::new()
        .command("healthcheck", command_healthcheck)
        .group("session", session::group())
        .group("auth", authentication::group())
}

fn command_healthcheck() -> String {
    interface::healthcheck()
        .unwrap_or(status::Healthcheck::Dead)
        .to_string()
}
