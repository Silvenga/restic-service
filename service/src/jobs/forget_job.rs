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

        // Retention policy options
        if let Some(group_by) = &self.config.group_by {
            options = options.group_by(group_by);
        }
        if let Some(keep_last) = self.config.keep_last {
            options = options.keep_last(keep_last);
        }
        if let Some(keep_hourly) = self.config.keep_hourly {
            options = options.keep_hourly(keep_hourly);
        }
        if let Some(keep_daily) = self.config.keep_daily {
            options = options.keep_daily(keep_daily);
        }
        if let Some(keep_weekly) = self.config.keep_weekly {
            options = options.keep_weekly(keep_weekly);
        }
        if let Some(keep_monthly) = self.config.keep_monthly {
            options = options.keep_monthly(keep_monthly);
        }
        if let Some(keep_yearly) = self.config.keep_yearly {
            options = options.keep_yearly(keep_yearly);
        }
        if let Some(keep_within) = &self.config.keep_within {
            options = options.keep_within(keep_within);
        }
        if let Some(keep_within_hourly) = &self.config.keep_within_hourly {
            options = options.keep_within_hourly(*keep_within_hourly);
        }
        if let Some(keep_within_daily) = &self.config.keep_within_daily {
            options = options.keep_within_daily(*keep_within_daily);
        }
        if let Some(keep_within_weekly) = &self.config.keep_within_weekly {
            options = options.keep_within_weekly(*keep_within_weekly);
        }
        if let Some(keep_within_monthly) = &self.config.keep_within_monthly {
            options = options.keep_within_monthly(*keep_within_monthly);
        }
        if let Some(keep_within_yearly) = &self.config.keep_within_yearly {
            options = options.keep_within_yearly(*keep_within_yearly);
        }
        if let Some(keep_tag) = &self.config.keep_tag {
            options = options.keep_tag(keep_tag);
        }

        // Filtering options
        if let Some(host) = &self.config.host {
            options = options.host(host);
        }
        if let Some(tag) = &self.config.tag {
            options = options.tag(tag);
        }
        if let Some(path) = &self.config.path {
            options = options.path(path);
        }

        // Behavior options
        if self.config.unsafe_allow_remove_all {
            options = options.unsafe_allow_remove_all();
        }
        if self.config.compact {
            options = options.compact();
        }
        if self.config.dry_run {
            options = options.dry_run();
        }
        if self.config.prune {
            options = options.prune();
        }

        // Prune-specific options
        if let Some(max_unused) = &self.config.max_unused {
            options = options.max_unused(max_unused);
        }
        if let Some(max_repack_size) = &self.config.max_repack_size {
            options = options.max_repack_size(max_repack_size);
        }
        if self.config.repack_cacheable_only {
            options = options.repack_cacheable_only();
        }
        if self.config.repack_small {
            options = options.repack_small();
        }
        if self.config.repack_uncompressed {
            options = options.repack_uncompressed();
        }
        if let Some(repack_smaller_than) = &self.config.repack_smaller_than {
            options = options.repack_smaller_than(repack_smaller_than);
        }

        // Additional flags for any custom options not covered above
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
