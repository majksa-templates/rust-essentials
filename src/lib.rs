mod builder;
mod context;

#[cfg(feature = "log")]
mod log;

#[cfg(feature = "dotenv")]
mod dotenv;

use builder::Builder;
pub use context::Context;
pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{util::SubscriberInitExt, Registry};

pub fn install() {
    Builder::<Registry>::default().install();
}

pub fn builder<L>() -> builder::Builder<L>
where
    L: SubscriberInitExt + Default,
{
    Builder::new()
}
