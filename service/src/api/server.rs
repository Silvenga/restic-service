use crate::api::endpoints::{get_job_by_id, get_jobs, health, queue_job_by_id};
use crate::api::state::ApiState;
use crate::jobs::JobManager;
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub async fn run_server(
    job_manager: &Arc<JobManager>,
    cancellation_token: &CancellationToken,
) -> std::io::Result<()> {
    let server_cancellation_token = cancellation_token.child_token();
    let server = HttpServer::new({
        let job_manager = job_manager.clone();
        move || {
            let api = web::scope("/api/v1")
                .app_data(web::Data::new(ApiState {
                    job_manager: job_manager.clone(),
                }))
                .service(health)
                .service(get_jobs)
                .service(get_job_by_id)
                .service(queue_job_by_id);
            App::new().service(api)
        }
    })
    .bind(("127.0.0.1", 42038))?
    .shutdown_signal(server_cancellation_token.cancelled_owned())
    .workers(2)
    .server_hostname("Restic Service")
    .run();

    server.await
}
