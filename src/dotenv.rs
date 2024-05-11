use dotenv::{dotenv, Error};
use tracing::error;

pub fn install() {
    if let Err(e) = dotenv() {
        match e {
            Error::Io(_) => (),
            _ => error!("Failed to load .env file: {}", e),
        };
    }
}
