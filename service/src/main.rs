mod cli;
mod host;
mod management;

#[macro_use]
extern crate windows_service;

use crate::cli::{Verb, parse_args};
use crate::host::run::run;
use crate::management::{
    SERVICE_NAME, install_service, restart_service, start_service, status_service, stop_service,
    uninstall_service,
};
use std::env;
use std::process::ExitCode;
use windows_service::service_dispatcher;

const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: i32 = 1063;

define_windows_service!(ffi_service_main, run);

#[cfg(windows)]
fn main() -> ExitCode {
    match service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
        // Likely running in CLI mode.
        Err(windows_service::Error::Winapi(io_err))
            if io_err.raw_os_error() == Some(ERROR_FAILED_SERVICE_CONTROLLER_CONNECT) =>
        {
            main_cli()
        }
        // Service ran and stopped gracefully.
        Ok(_) => ExitCode::SUCCESS,
        // Service failed to start.
        Err(_) => ExitCode::FAILURE,
    }
}

fn main_cli() -> ExitCode {
    let args = parse_args();

    if args.verb == Verb::Run {
        run(env::args_os().collect());
        return ExitCode::SUCCESS;
    }

    let service_result = match args.verb {
        Verb::Install => install_service(),
        Verb::Uninstall => uninstall_service(),
        Verb::Start => start_service(),
        Verb::Stop => stop_service(),
        Verb::Restart => restart_service(),
        Verb::Status => status_service(),
        Verb::Run => {
            panic!("Impossible case.")
        }
    };

    match service_result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
