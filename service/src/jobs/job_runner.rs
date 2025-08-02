use super::forget_job::ForgetJob;
use crate::config::ResticJob;
use crate::jobs::backup_job::BackupJob;
use crate::jobs::clear_locks::ClearLocksJob;
use log::{info, warn};
use restic_sdk::errors::ResticError;
use restic_sdk::{Restic, ResticConfig};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

pub struct JobRunner {}

impl JobRunner {
    pub async fn run(job_config: &ResticJob, cancellation_token: &CancellationToken) {
        let client = Self::build_restic_client(job_config);
        Self::run_job(
            &client,
            BackupJob::new(&job_config.backup),
            cancellation_token,
        )
        .await;
        Self::run_job(
            &client,
            ClearLocksJob::new(&job_config.clear_locks),
            cancellation_token,
        )
        .await;
        Self::run_job(
            &client,
            ForgetJob::new(&job_config.forget_and_purge),
            cancellation_token,
        )
        .await;
    }

    async fn run_job(
        client: &Restic,
        job: impl RunnableJob,
        cancellation_token: &CancellationToken,
    ) {
        let job_name = job.get_job_name();
        let start = Instant::now();

        info!("Running {job_name}...");
        let result = job.run(client, cancellation_token).await;

        match result {
            Ok(_) => {
                start.elapsed();
                info!(
                    "{job_name} completed successfully in {:?}.",
                    start.elapsed()
                );
            }
            Err(e) => {
                warn!("{job_name} failed with error: {e:?}");
            }
        }
    }

    fn build_restic_client(job_config: &ResticJob) -> Restic {
        let mut restic_config = ResticConfig::default()
            .with_repository(&job_config.repository)
            .with_password(&job_config.password);

        for (env_name, env_value) in &job_config.environment {
            restic_config = restic_config.with_env_var(env_name, env_value);
        }

        Restic::new().with_config(restic_config)
    }
}

pub trait RunnableJob {
    async fn run(
        &self,
        client: &Restic,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError>;

    fn get_job_name(&self) -> &str;
}
