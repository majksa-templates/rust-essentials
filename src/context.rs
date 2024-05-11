use serde::{Deserialize, Deserializer};
use serde_env::from_env;

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "p" | "prod" | "production" => Self::Production,
            "d" | "dev" | "development" | _ => Self::Development,
        }
    }
}

impl<'de> Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Environment, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.into())
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Context {
    #[serde(default)]
    pub app_env: Environment,
}

impl Context {
    pub fn load() -> Self {
        from_env().unwrap()
    }
}
