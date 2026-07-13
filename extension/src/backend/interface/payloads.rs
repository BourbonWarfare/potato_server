use serde::{Deserialize, Serialize};

use crate::orbat::Orbat;

#[derive(Serialize, Deserialize)]
pub struct FinishMission {
    pub session_id: uuid::Uuid,
    pub mission_name_with_version: String,
    pub mission_map: String,
    pub starting_orbat: Orbat,
    pub final_orbat: Orbat,
}

#[derive(Serialize, Deserialize)]
pub struct SafeStartEnd {
    pub session_id: uuid::Uuid,
    pub mission_name_with_version: String,
    pub mission_map: String,
    pub orbat: Orbat,
}

#[derive(Serialize, Deserialize)]
pub struct LoginBot {
    pub bot_token: String,
}
