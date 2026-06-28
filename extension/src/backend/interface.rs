use crate::backend::status;
use crate::error::ArmaError;
use crate::error::healthcheck::HealthcheckError;
use url::Url;

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
