#[derive(Debug, Copy, Clone)]
pub enum Healthcheck {
    Healthy,
    Dead,
}

impl ToString for Healthcheck {
    fn to_string(&self) -> String {
        match self {
            Healthcheck::Healthy => String::from("Healthy"),
            Healthcheck::Dead => String::from("Dead"),
        }
    }
}
