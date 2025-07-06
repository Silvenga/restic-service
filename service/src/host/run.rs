use std::ffi::OsString;
use tokio_util::sync::CancellationToken;

pub async fn run(_arguments: Vec<OsString>, _cancellation_token: CancellationToken) -> u8 {
    // The entry point where execution will start on a background thread after a call to
    // `service_dispatcher::start` from `main`.
    0
}
