use crate::config::{ResticJob, ServiceConfiguration};
use log::info;
use thiserror::Error;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;

pub struct JobManager {
    config: ServiceConfiguration,
    sender: Sender<(String, ResticJob)>,
}

impl JobManager {
    pub fn new(config: ServiceConfiguration, sender: Sender<(String, ResticJob)>) -> Self {
        Self { config, sender }
    }

    pub fn get_jobs(&self) -> Vec<(String, ResticJob)> {
        self.config
            .jobs
            .iter()
            .map(|(job_id, job_config)| (job_id.clone(), job_config.clone()))
            .collect()
    }

    pub async fn queue_job(&self, job_id: impl Into<String>) -> Result<(), QueueJobError> {
        let job_id = job_id.into();

        let Some(job) = self.config.jobs.get(&job_id) else {
            return Err(QueueJobError::JobNotFound(job_id));
        };

        self.sender
            .send((job_id.clone(), job.clone()))
            .await
            .map_err(QueueJobError::QueueSendError)?;

        info!("Job '{job_id}' is queued.");

        Ok(())
    }
}

#[derive(Error, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum QueueJobError {
    #[error("job {0} not found")]
    JobNotFound(String),
    #[error("failed to send job to queue")]
    QueueSendError(SendError<(String, ResticJob)>),
}
