use arma_rs::{FromArma, Group, IntoArma};

use crate::error::{ArmaError, authentication as auth};

pub fn group() -> Group {
    Group::new()
        .command("login", command_login)
        .command("refresh", command_refresh_session)
}

fn command_login(login: Login) -> Result<Session, ArmaError<auth::LoginError>> {
    login.try_create_session()
}

fn command_refresh_session(session: Session) -> Result<Session, ArmaError<auth::SessionError>> {
    session.try_refresh()
}

#[derive(FromArma, Debug, Clone)]
pub struct Login(String);

#[derive(IntoArma, FromArma, Debug, Clone)]
pub struct Session {
    token: String,
    expires: u32,
}

impl Login {
    pub fn try_create_session(&self) -> Result<Session, ArmaError<auth::LoginError>> {
        Err(auth::LoginError::InvalidBotToken.into())
    }
}

impl Session {
    pub fn try_refresh(self) -> Result<Session, ArmaError<auth::SessionError>> {
        Err(auth::SessionError::CouldNotRefresh.into())
    }
}
