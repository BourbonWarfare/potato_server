use arma_rs::{FromArma, Group, IntoArma};

use crate::backend::map::Map;
use crate::backend::mission::Mission;
use crate::backend::session::Session as AuthSession;
use crate::http::Status;

#[derive(FromArma, IntoArma)]
pub struct Session {
    id: uuid::Uuid,
}

pub fn group() -> Group {
    Group::new()
        .command("current", command_current)
        .command("finish_mission", command_finish_mission)
}

fn command_current(auth: AuthSession) -> Session {
    Session {
        id: uuid::Uuid::new_v4(),
    }
}

fn command_finish_mission(
    auth: AuthSession,
    session: Session,
    mission: Mission,
    map: Map,
    player_count: i64,
) -> Result<(), Status> {
    Err(Status(500))
}
