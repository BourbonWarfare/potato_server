use serde::Serialize;

#[derive(Serialize)]
pub struct FinishMission {
    pub session_id: uuid::Uuid,
    pub mission_name_with_version: String,
    pub mission_map: String,
    pub player_count: u32,
}

#[derive(Serialize)]
pub struct LoginBot {
    pub bot_token: String,
}
