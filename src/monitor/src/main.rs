use std::process::ExitCode;

#[cfg(windows)]
fn main() -> ExitCode {
    log_panics::init();

    ExitCode::SUCCESS
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
