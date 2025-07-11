use crate::config::BackupJobConfiguration;
use crate::jobs::RunnableJob;
use log::{info, warn};
use restic_sdk::Restic;
use restic_sdk::backup::BackupOptions;
use restic_sdk::errors::ResticError;
use sysinfo::Disks;
use tokio::fs::{canonicalize, try_exists};
use tokio_util::sync::CancellationToken;

pub struct BackupJob {
    config: BackupJobConfiguration,
}

impl BackupJob {
    pub fn new(config: &BackupJobConfiguration) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn build_backup_options(&self) -> BackupOptions {
        let mut options = BackupOptions::default();

        if self.config.exclude_caches {
            options = options.with_exclude_caches();
        }

        if self.config.verbose {
            options = options.with_verbose();
        }

        if self.config.use_fs_snapshot {
            options = options.use_fs_snapshot();
        }

        if self.config.cleanup_cache {
            options = options.with_cleanup_cache();
        }

        #[cfg(not(windows))]
        if self.config.one_file_system {
            options = options.with_one_file_system();
        }

        // Additional flags for any custom options not covered above
        for flag in &self.config.additional_flags {
            options = options.with_flag(flag);
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
}

impl RunnableJob for BackupJob {
    async fn run(
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

        let backup_options = self.build_backup_options();
        let sources = self.get_backup_sources().await;

        info!("Will backup [{:?}]...", sources.join(", "));
        let backup_result = client
            .backup(sources, backup_options, cancellation_token)
            .await?;

        info!("Backup completed successfully with summary {backup_result:?}");

        Ok(())
    }

    fn get_job_name(&self) -> &str {
        "Backup"
    }
}
