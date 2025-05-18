use std::ffi::OsString;

#[tokio::main]
pub async fn run(_arguments: Vec<OsString>) {
    // The entry point where execution will start on a background thread after a call to
    // `service_dispatcher::start` from `main`.
}
