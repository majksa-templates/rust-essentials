use std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s {
            "development" => Self::Development,
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

impl From<Option<String>> for Environment {
    fn from(e: Option<String>) -> Self {
        match e {
            Some(e) => e.as_str().into(),
            None => Self::Development,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub environment: Environment,
}

impl Context {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    pub fn load() -> Self {
        Self {
            environment: env::var("APP_ENV").ok().into(),
        }
    }
}
