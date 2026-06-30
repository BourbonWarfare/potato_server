use crate::backend::interface::payloads::LoginBot;
use arma_rs::{FromArma, FromArmaError, Group, IntoArma, Value};
use chrono::{DateTime, Local, Utc};
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
    let expire_time = session.expire_time - Local::now().to_utc();
    expire_time.as_seconds_f64()
}

#[derive(FromArma, Debug, Clone)]
pub struct Login(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub session_token: String,
    expire_time: DateTime<Utc>,
}

impl IntoArma for Session {
    fn to_arma(&self) -> Value {
        Value::Array(vec![
            Value::String(self.session_token.clone()),
            Value::String(self.expire_time.to_rfc3339()),
        ])
    }
}

impl FromArma for Session {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let Value::Array(array) = Value::from_arma(s)? else {
            return Err(FromArmaError::InvalidValue(
                "Session object requires array".to_string(),
            ));
        };

        if array.len() != 2 {
            return Err(FromArmaError::InvalidLength {
                expected: 2,
                actual: array.len(),
            });
        }

        let Value::String(session_token) = array[0].clone() else {
            return Err(FromArmaError::InvalidValue(
                "First array element is not String".to_string(),
            ));
        };

        let Value::String(expire_time) = array[1].clone() else {
            return Err(FromArmaError::InvalidValue(
                "Second array element is not String".to_string(),
            ));
        };

        let expire_time = DateTime::parse_from_rfc3339(expire_time.as_str())
            .map_err(|_| {
                FromArmaError::InvalidValue(
                    "Expire Time needs to be valid ISO8601 string!".to_string(),
                )
            })?
            .to_utc();

        Ok(Session {
            session_token,
            expire_time,
        })
    }
}

impl Login {
    pub fn try_create_session(&self) -> Result<Session, ArmaError<auth::LoginError>> {
        interface::login_bot(LoginBot {
            bot_token: self.0.clone(),
        })
    }
}
