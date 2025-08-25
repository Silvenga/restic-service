use crate::api::endpoints::{get_job_by_id, get_jobs, health, queue_job_by_id};
use crate::api::state::ApiState;
use crate::jobs::JobManager;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use common::config::ApiConfiguration;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub async fn run_server(
    config: &ApiConfiguration,
    job_manager: &Arc<JobManager>,
    cancellation_token: &CancellationToken,
) -> std::io::Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let server_cancellation_token = cancellation_token.child_token();
    let server = HttpServer::new({
        let job_manager = job_manager.clone();
        move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_header()
                .allow_any_method()
                .max_age(3600);

            let api = web::scope("/api/v1")
                .wrap(cors)
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
    .bind((config.host.clone(), config.port))?
    .shutdown_signal(server_cancellation_token.cancelled_owned())
    .workers(config.workers)
    .server_hostname(config.host.clone())
    .run();

    server.await
}
