//! Example of a simple library
//!
//! # Examples
//!
//! ```
//! use rust_lib::add;
//!
//! fn main() {
//!     let result = add(2, 2);
//!     assert_eq!(result, 4);
//! }
//! ```

use tracing::warn;
#[cfg(feature = "panic")]
use tracing_panic::panic_hook;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use crate::{context::Environment, Context};

pub fn install<L>(context: &Context, tracing: Option<L>)
where
    L: SubscriberInitExt,
{
    let is_development = context.environment == Environment::Development;
    if let Some(tracing) = tracing {
        tracing.init();
    } else if is_development {
        tracing_subscriber::registry()
            .with(fmt::layer().pretty())
            .with(EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().json())
            .with(EnvFilter::from_default_env())
            .init();
    }
    if is_development {
        if color_eyre::install().is_err() {
            warn!("Failed to install color_eyre");
        }
    }
    #[cfg(feature = "panic")]
    std::panic::set_hook(Box::new(panic_hook));
}
