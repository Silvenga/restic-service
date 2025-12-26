#[macro_use]
extern crate windows_service;
mod api;
pub(crate) mod cli;
mod host;
mod jobs;
pub(crate) mod management;
pub(crate) mod paths;
pub(crate) mod service;

use crate::cli::{Verb, parse_args};
use crate::host::ServiceHost;
use crate::management::{
    SERVICE_NAME, install_service, restart_service, start_service, status_service, stop_service,
    uninstall_service,
};
use crate::paths::get_exe_directory;
use crate::service::ServiceStatusHandlerExtension;
use flexi_logger::{
    AdaptiveFormat, Cleanup, Criterion, FileSpec, Logger, Naming, WriteMode, detailed_format,
};
use log::info;
use std::env;
use std::ffi::OsString;
use std::process::ExitCode;
use tokio_util::sync::CancellationToken;
use windows_service::service::{ServiceControl, ServiceControlAccept};
use windows_service::service_control_handler::ServiceControlHandlerResult;
use windows_service::{service_control_handler, service_dispatcher};

const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: i32 = 1063;

define_windows_service!(ffi_service_main, main_service);

#[cfg(windows)]
fn main() -> ExitCode {
    log_panics::init();

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

#[tokio::main]
async fn main_cli() -> ExitCode {
    let _logger = Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_stdout()
        .use_windows_line_ending()
        .write_mode(WriteMode::BufferAndFlush)
        .append()
        .adaptive_format_for_stdout(AdaptiveFormat::Default)
        .start()
        .unwrap();

    let args = parse_args();

    let service_result = match args.verb {
        Verb::Install => install_service(),
        Verb::Uninstall => uninstall_service(),
        Verb::Start => start_service(),
        Verb::Stop => stop_service(),
        Verb::Restart => restart_service(),
        Verb::Status => status_service(),
        Verb::Run => {
            info!("Running service in CLI mode...");

            let cancellation_token = CancellationToken::new();

            ctrlc::set_handler({
                let cancellation_token = cancellation_token.clone();
                move || {
                    info!("Ctrl+C received, canceling jobs...");
                    cancellation_token.cancel();
                }
            })
            .expect("handler to be set");

            let exit_code = ServiceHost::run(env::args_os().collect(), &cancellation_token).await;

            info!("Service exited with code {exit_code}.");

            return ExitCode::from(exit_code);
        }
    };

    match service_result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn main_service(arguments: Vec<OsString>) {
    let _logger = Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .basename("restic-service")
                .directory(get_exe_directory())
                .suppress_timestamp(),
        )
        .use_windows_line_ending()
        .write_mode(WriteMode::BufferAndFlush)
        .append()
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::NumbersDirect,
            Cleanup::KeepLogFiles(3),
        )
        .format_for_files(detailed_format)
        .start()
        .unwrap();

    // Service Pre-Start, not under SCM control.
    let cancellation_token = CancellationToken::new();
    let status_handle = service_control_handler::register(SERVICE_NAME, {
        let token = cancellation_token.clone();
        move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                ServiceControl::Stop => {
                    token.cancel();
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        }
    })
    .expect("register service control handler should always succeed");

    let cancellation_task = tokio::spawn({
        let cancellation_token = cancellation_token.clone();
        async move {
            cancellation_token.cancelled().await;
            info!("Stop requested, canceling jobs...");
            status_handle
                .set_status_stop_pending()
                .expect("set_status_stop_pending should always succeed");
        }
    });

    info!("Service started, running jobs...");

    status_handle
        .set_status_running(ServiceControlAccept::STOP)
        .expect("set service status should always succeed");

    let exit_code = ServiceHost::run(arguments, &cancellation_token).await;
    cancellation_token.cancel();

    cancellation_task.await.unwrap();

    status_handle
        .set_status_stopped(exit_code as u32)
        .expect("set status_stopped should always succeed");
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
