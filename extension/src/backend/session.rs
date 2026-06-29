use arma_rs::{FromArma, Group, IntoArma};
use serde::{Deserialize, Serialize};

use crate::backend::authentication::Session as AuthSession;
use crate::backend::interface;
use crate::backend::interface::payloads::FinishMission;
use crate::error::ArmaError;
use crate::error::session;

#[derive(FromArma, IntoArma, Deserialize, Serialize)]
pub struct Session {
    id: uuid::Uuid,
}

pub fn group() -> Group {
    Group::new()
        .command("current", command_current)
        .command("finish_mission", command_finish_mission)
}

fn command_current(auth: AuthSession) -> Result<Session, ArmaError<session::SessionError>> {
    interface::get_current_session(auth)
}

fn command_finish_mission(
    auth: AuthSession,
    session: Session,
    mission: String,
    map: String,
    player_count: u32,
) -> Result<(), ArmaError<session::SessionError>> {
    interface::finish_mission(
        auth,
        FinishMission {
            session_id: session.id,
            mission_name_with_version: mission,
            mission_map: map,
            player_count,
        },
    )
}
