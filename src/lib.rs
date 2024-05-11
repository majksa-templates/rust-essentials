mod builder;
mod context;

#[cfg(feature = "log")]
mod log;

use builder::Builder;
pub use context::Context;
pub use tracing;
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
