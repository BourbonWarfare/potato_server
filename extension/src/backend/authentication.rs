use crate::backend::interface::payloads::LoginBot;
use arma_rs::{FromArma, Group, IntoArma};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    backend::interface,
    error::{ArmaError, authentication as auth},
};

pub fn group() -> Group {
    Group::new()
        .command("login", command_login)
        .command("session_expire_time", command_session_expire_time)
}

fn command_login(login: Login) -> Result<Session, ArmaError<auth::LoginError>> {
    login.try_create_session()
}

fn command_session_expire_time(session: Session) -> f64 {
    let expire_time = session.expire_time.and_utc() - Local::now().to_utc();
    expire_time.as_seconds_f64()
}

#[derive(FromArma, Debug, Clone)]
pub struct Login(String);

#[derive(Deserialize, Serialize, IntoArma, FromArma, Debug, Clone)]
pub struct Session {
    pub session_token: String,
    expire_time: NaiveDateTime,
}

impl Login {
    pub fn try_create_session(&self) -> Result<Session, ArmaError<auth::LoginError>> {
        interface::login_bot(LoginBot {
            bot_token: self.0.clone(),
        })
    }
}
