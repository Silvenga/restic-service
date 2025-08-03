use crate::api::endpoints::health;
use actix_web::{App, HttpServer};
use tokio_util::sync::CancellationToken;

pub async fn run_server(cancellation_token: &CancellationToken) -> std::io::Result<()> {
    let server_cancellation_token = cancellation_token.child_token();
    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 42038))?
        .shutdown_signal(server_cancellation_token.cancelled_owned())
        .workers(2)
        .server_hostname("Restic Service")
        .run()
        .await
}
