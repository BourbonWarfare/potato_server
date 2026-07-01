use arma_rs::{FromArma, IntoArma};
use serde::{Deserialize, Serialize};

#[derive(FromArma, IntoArma, Deserialize, Serialize, Debug, Clone)]
#[repr(transparent)]
pub struct SteamId(String);

#[derive(FromArma, IntoArma, Deserialize, Serialize, Debug, Clone)]
pub struct Individual {
    variable: String,
    name: String,
    is_member: bool,
    rank: u32,
    steam_id: SteamId,
}

#[derive(FromArma, IntoArma, Deserialize, Serialize, Debug, Clone)]
pub struct Group {
    name: String,
    side: String,
    leader: SteamId,
    members: Vec<Individual>,
}

#[derive(FromArma, IntoArma, Deserialize, Serialize, Debug, Clone)]
pub struct Orbat {
    groups: Vec<Group>,
}
