use crate::config::ForgetConfiguration;
use crate::jobs::RunnableJob;
use log::info;
use restic_sdk::Restic;
use restic_sdk::errors::ResticError;
use restic_sdk::forget::ForgetOptions;
use tokio_util::sync::CancellationToken;

pub struct ForgetJob {
    config: ForgetConfiguration,
}

impl ForgetJob {
    pub fn new(config: &ForgetConfiguration) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn get_forget_and_prune_options(&self) -> ForgetOptions {
        let mut options = ForgetOptions::default();

        options = options.prune();

        for flag in &self.config.additional_flags {
            options = options.with_flag(flag);
        }

        options
    }
}

impl RunnableJob for ForgetJob {
    async fn run(
        &self,
        client: &Restic,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError> {
        if !self.config.enabled {
            info!("Forget and prune is disabled by configuration.");
            return Ok(());
        }

        let forget_options = self.get_forget_and_prune_options();
        client.forget(forget_options, cancellation_token).await?;

        Ok(())
    }

    fn get_job_name(&self) -> &str {
        "Forget and Prune"
    }
}
