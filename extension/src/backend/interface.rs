use crate::backend::authentication::{Login, Session as AuthSession};
use crate::backend::session::Session;
use crate::backend::status;
use crate::error::ArmaError;
use crate::error::authentication::LoginError;
use crate::error::healthcheck::HealthcheckError;
use crate::error::session::SessionError;
use url::Url;

pub mod payloads;

fn server_url() -> Url {
    Url::parse("https://localhost").expect("URL location needs to be a valid URL format")
}

fn api_url(path: impl AsRef<str>) -> Url {
    const API_PATH: &str = "api/v1/";
    server_url()
        .join(API_PATH)
        .expect("Programming error: Constant `API_PATH` is not a valid URL format")
        .join(path.as_ref())
        .expect("API location needs to be a valid URL format")
}

pub fn healthcheck() -> Result<status::Healthcheck, ArmaError<HealthcheckError>> {
    let url = server_url()
        .join("healthcheck")
        .expect("Programming error: invalid healthcheck URL");

    reqwest::blocking::get(url).map_err(|_| ArmaError::from(HealthcheckError::Failed))?;
    Ok(status::Healthcheck::Healthy)
}

pub fn login_bot(payload: payloads::LoginBot) -> Result<AuthSession, ArmaError<LoginError>> {
    let url = api_url("auth/login/bot");
    let response: AuthSession = reqwest::blocking::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .map_err(|_| ArmaError::from(LoginError::InvalidBotToken))?
        .json()
        .map_err(|_| ArmaError::from(LoginError::CouldNotLogin))?;
    Ok(response)
}

pub fn get_current_session(auth: AuthSession) -> Result<Session, ArmaError<SessionError>> {
    let url = api_url("sessions/current");
    let response: Session = reqwest::blocking::Client::new()
        .get(url)
        .bearer_auth(auth.session_token)
        .send()
        .map_err(|_| ArmaError::from(SessionError::NoCurrentSession))?
        .json()
        .map_err(|_| ArmaError::from(SessionError::CouldNotGetSession))?;
    Ok(response)
}

pub fn finish_mission(
    auth: AuthSession,
    payload: payloads::FinishMission,
) -> Result<(), ArmaError<SessionError>> {
    let url = api_url("sessions/mission/finish");
    reqwest::blocking::Client::new()
        .post(url)
        .bearer_auth(auth.session_token)
        .json(&payload)
        .send()
        .map_err(|err| {
            ArmaError::from(if let Some(status) = err.status() {
                match status.as_u16() {
                    400 => SessionError::InsufficientPlayercount,
                    404 => SessionError::NoMissionExists,
                    _ => SessionError::CouldNotGetSession,
                }
            } else {
                SessionError::CouldNotGetSession
            })
        })?;
    Ok(())
}
