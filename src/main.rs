use essentials::tracing::*;

fn main() {
    essentials::install();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
