use crate::config::ResticJob;
use log::{info, warn};
use restic_sdk::backup::BackupOptions;
use restic_sdk::errors::ResticError;
use restic_sdk::forget::ForgetOptions;
use restic_sdk::{Restic, ResticConfig};
use sysinfo::Disks;
use tokio::fs::{canonicalize, try_exists};
use tokio_util::sync::CancellationToken;

pub struct JobRunner {
    config: ResticJob,
}

impl JobRunner {
    pub fn new(config: ResticJob) -> Self {
        JobRunner { config }
    }

    pub async fn run(&self, cancellation_token: &CancellationToken) {
        let client = self.get_restic_client();

        let result = self.run_backup(&client, cancellation_token).await;
        if let Err(e) = result {
            warn!("Backup failed with error: {e:?}");
        }

        if self.config.forget_and_purge.enabled {
            let result = self.run_forget_and_prune(&client, cancellation_token).await;
            if let Err(e) = result {
                warn!("Forget and prune failed with error: {e:?}");
            }
        }
    }

    async fn run_backup(
        &self,
        client: &Restic,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError> {
        if !client.can_open(cancellation_token).await? {
            info!("Restic repository cannot be opened, assuming it does not exist.");
            let result = client.init(cancellation_token).await?;
            info!(
                "Restic repository initialized successfully with id {:?}",
                result.id
            );
        }

        let backup_options = self.get_backup_options();
        let sources = self.get_backup_sources().await;

        info!("Starting backup against {:?}...", sources.join(", "));
        let backup_result = client
            .backup(sources, backup_options, cancellation_token)
            .await?;

        info!("Backup completed successfully with summary {backup_result:?}");

        Ok(())
    }

    async fn run_forget_and_prune(
        &self,
        client: &Restic,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError> {
        info!("Starting forget and prune...");

        let forget_options = self.get_forget_and_prune_options();
        client.forget(forget_options, cancellation_token).await?;

        info!("Prune completed successfully.");

        Ok(())
    }

    fn get_restic_client(&self) -> Restic {
        let mut restic_config = ResticConfig::default()
            .with_repository(&self.config.repository)
            .with_password(&self.config.password);

        for (env_name, env_value) in &self.config.environment {
            restic_config = restic_config.with_env_var(env_name, env_value);
        }

        Restic::new().with_config(restic_config)
    }

    fn get_backup_options(&self) -> BackupOptions {
        let mut options = BackupOptions::default();

        if self.config.backup.exclude_caches {
            options = options.with_exclude_caches();
        }

        if self.config.backup.verbose {
            options = options.with_verbose();
        }

        if self.config.backup.use_fs_snapshot {
            options = options.use_fs_snapshot();
        }

        options
    }

    async fn get_backup_sources(&self) -> Vec<String> {
        let mut sources = Vec::new();

        // Source explicit sources.
        for source in &self.config.sources {
            if try_exists(source).await.unwrap_or_else(|_| {
                warn!("Failed to check existence of source path '{source}', it will be ignored.");
                false
            }) {
                match canonicalize(source).await {
                    Ok(_) => {
                        sources.push(source.clone());
                    }
                    Err(_) => {
                        warn!("Failed to canonicalize source path '{source}', it will be ignored.");
                    }
                };
            }
        }

        // Source fixed drives if configured.
        if self.config.source_fixed_drives {
            let disks = Disks::new_with_refreshed_list();
            for disk in disks.list() {
                if !disk.is_removable() {
                    if let Some(mount_point) = disk.mount_point().to_str() {
                        info!("Adding disk source: '{mount_point:?}'");
                        sources.push(mount_point.to_owned());
                    } else {
                        warn!("Failed to convert disk '{disk:?}' to string, it will be ignored.");
                    }
                }
            }
        }

        sources
    }

    fn get_forget_and_prune_options(&self) -> ForgetOptions {
        let mut options = ForgetOptions::default();

        options = options.prune();

        for flag in &self.config.forget_and_purge.additional_flags {
            options = options.with_flag(flag);
        }

        options
    }
}
