use crate::api::errors::AppApiError;
use crate::api::state::ApiState;
use crate::config::ResticJob;
use crate::jobs::QueueJobError;
use actix_web::{get, post, web};
use log::warn;
use serde::Serialize;
use std::collections::HashMap;

#[get("/jobs")]
pub async fn get_jobs(data: web::Data<ApiState>) -> web::Json<GetJobsResponse> {
    let jobs = data.job_manager.get_jobs().into_iter().collect();
    web::Json(jobs)
}

#[get("/jobs/{id}")]
pub async fn get_job_by_id(
    path: web::Path<String>,
    data: web::Data<ApiState>,
) -> Result<web::Json<GetJobByIdResponse>, AppApiError> {
    let id = path.into_inner();
    let jobs: Vec<_> = data
        .job_manager
        .get_jobs()
        .into_iter()
        .filter(|(job_id, _)| job_id == &id)
        .map(|(job_id, job)| GetJobByIdResponse { job_id, job })
        .collect();

    if jobs.is_empty() {
        return Err(AppApiError::JobNotFound);
    }

    Ok(web::Json(jobs.into_iter().next().unwrap()))
}

#[post("/jobs/{id}/queue")]
pub async fn queue_job_by_id(
    path: web::Path<String>,
    data: web::Data<ApiState>,
) -> Result<web::Json<()>, AppApiError> {
    let id = path.into_inner();
    match data.job_manager.queue_job(id).await {
        Ok(_) => Ok(web::Json(())),
        Err(QueueJobError::JobNotFound(_)) => Err(AppApiError::JobNotFound),
        Err(QueueJobError::QueueSendError(e)) => {
            warn!(
                "Failed to queue job, the job queue might be full or the service is shutting down. Error: {e:?}"
            );
            Err(AppApiError::InternalServerError)
        }
    }
}

pub type GetJobsResponse = HashMap<String, ResticJob>;

#[derive(Serialize)]
pub struct GetJobByIdResponse {
    job_id: String,
    job: ResticJob,
}
