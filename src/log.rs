use tracing::{level_filters::LevelFilter, warn};
#[cfg(feature = "panic")]
use tracing_panic::panic_hook;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use crate::{context::Environment, Context};

pub fn install<L>(context: &Context, tracing: Option<L>)
where
    L: SubscriberInitExt,
{
    let is_development = context.app_env == Environment::Development;
    if let Some(tracing) = tracing {
        tracing.init();
    } else if is_development {
        tracing_subscriber::registry()
            .with(fmt::layer().pretty())
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::TRACE.into())
                    .from_env_lossy(),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().json())
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .init();
    }
    if is_development & color_eyre::install().is_err() {
        warn!("Failed to install color_eyre");
    }
    #[cfg(feature = "panic")]
    std::panic::set_hook(Box::new(panic_hook));
}
