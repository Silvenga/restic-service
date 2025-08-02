use crate::config::ClearLocksJobConfiguration;
use crate::jobs::RunnableJob;
use log::{debug, info, warn};
use restic_sdk::Restic;
use restic_sdk::errors::ResticError;
use tokio_util::sync::CancellationToken;

pub struct ClearLocksJob {
    config: ClearLocksJobConfiguration,
}

impl ClearLocksJob {
    pub fn new(config: &ClearLocksJobConfiguration) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl RunnableJob for ClearLocksJob {
    async fn run(
        &self,
        client: &Restic,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError> {
        if !self.config.enabled {
            info!("Removing stale locks is disabled by configuration.");
            return Ok(());
        }

        if !client.can_open(cancellation_token).await? {
            info!("Ignoring unlock attempt because the repository cannot be opened.");
            return Ok(());
        }

        info!("Checking for stale locks...");

        let lock_count = client
            .get_locks(cancellation_token)
            .await?
            .iter()
            .inspect(|x| {
                debug!("Found: {x}");
            })
            .count();

        if lock_count > 0 {
            info!("Found {lock_count} stale lock(s), attempting to remove them...");
            match client.unlock(cancellation_token).await {
                Ok(_) => info!("Successfully removed stale locks."),
                Err(e) => warn!("Failed to remove stale locks: {e}"),
            }
        } else {
            info!("No stale locks found.");
        }

        Ok(())
    }

    fn get_job_name(&self) -> &str {
        "Remove Stale Locks"
    }
}
