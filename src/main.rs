use tracing::{debug, error, info, trace, warn};

fn main() {
    essentials::install();
    debug!("Hello, world!");
    warn!("This is a warning");
    info!("This is an info message");
    error!("This is an error message");
    trace!("This is a trace message");
    panic!("This is a panic message");
}
