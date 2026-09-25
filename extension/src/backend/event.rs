use crate::backend::authentication::Session as AuthSession;
use crate::backend::interface;
use crate::backend::interface::payloads::Event;
use crate::error::ArmaError;
use crate::error::session;
use arma_rs::Group;

pub fn group() -> Group {
    Group::new().command("send", command_send)
}

fn command_send(
    auth: AuthSession,
    tag: String,
    message: String,
) -> Result<(), ArmaError<session::SessionError>> {
    interface::send_event(auth, Event { tag, message })
}
