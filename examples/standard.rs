use log::{debug, error, info, trace, warn, Level};
use simple_clap_logger::Logger;

fn main() {
    Logger::init_with_level(Level::Trace);

    error!("A error message");
    warn!("Danger!");
    info!("This program is currently running");
    debug!("Super important debug message");
    trace!("Adding 1 + 1");
}
