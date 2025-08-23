use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The operation to perform in cli mode.
    #[arg(value_enum)]
    pub verb: Verb,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Verb {
    /// Install (register) the service.
    Install,
    /// Uninstall (deregister) the service.
    Uninstall,
    /// Start the service, errors if the service is not installed.
    Start,
    /// Stop the service, errors if the service is not installed.
    Stop,
    /// Restart the service (or starts the service if stopped), errors if the service is not installed.
    Restart,
    /// Get the service status.
    Status,
    /// Run the service in the foreground.
    Run,
}

pub fn parse_args() -> Args {
    Args::parse()
}
