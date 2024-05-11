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

mod builder;
mod context;

#[cfg(feature = "log")]
mod log;

use builder::Builder;
pub use context::Context;
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
