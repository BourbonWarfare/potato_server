use crate::orbat::Orbat;
use arma_rs::Group;

use crate::backend::authentication::Session as AuthSession;
use crate::backend::session::Session;

pub fn group() -> Group {
    Group::new().command("safestart_off", command_safestart_off)
}

fn command_safestart_off(auth: AuthSession, session: Session, orbat: Orbat) {}
