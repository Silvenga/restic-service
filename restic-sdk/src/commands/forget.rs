use crate::errors::ResticError;
use crate::{ArgumentsBuilder, BuilderValue, Restic};
use log::debug;

impl Restic {
    pub async fn forget(&self, options: ForgetOptions) -> Result<(), ResticError> {
        self.exec(
            options.builder.with_verb("forget"),
            |string, output_type| {
                debug!("Restic {output_type:?}: {string}");
            },
        )
        .await?;
        Ok(())
    }
}

/// Options for the `restic forget` command.
#[derive(Debug, Clone, Default)]
pub struct ForgetOptions {
    builder: ArgumentsBuilder,
}

impl ForgetOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `--group-by` flag.
    ///
    /// Group snapshots by host, paths and/or tags, separated by comma.
    pub fn group_by(self, value: &str) -> Self {
        self.with_flag_and_value("group-by", value)
    }

    /// Sets the `--keep-last` flag.
    ///
    /// Keep the last `n` snapshots.
    pub fn keep_last(self, value: u32) -> Self {
        self.with_flag_and_value("keep-last", value)
    }

    /// Sets the `--keep-hourly` flag.
    ///
    /// Keep the last `n` hourly snapshots.
    pub fn keep_hourly(self, value: u32) -> Self {
        self.with_flag_and_value("keep-hourly", value)
    }

    /// Sets the `--keep-daily` flag.
    ///
    /// Keep the last `n` daily snapshots.
    pub fn keep_daily(self, value: u32) -> Self {
        self.with_flag_and_value("keep-daily", value)
    }

    /// Sets the `--keep-weekly` flag.
    ///
    /// Keep the last `n` weekly snapshots.
    pub fn keep_weekly(self, value: u32) -> Self {
        self.with_flag_and_value("keep-weekly", value)
    }

    /// Sets the `--keep-monthly` flag.
    ///
    /// Keep the last `n` monthly snapshots.
    pub fn keep_monthly(self, value: u32) -> Self {
        self.with_flag_and_value("keep-monthly", value)
    }

    /// Sets the `--keep-yearly` flag.
    ///
    /// Keep the last `n` yearly snapshots.
    pub fn keep_yearly(self, value: u32) -> Self {
        self.with_flag_and_value("keep-yearly", value)
    }

    /// Sets the `--keep-within` flag.
    ///
    /// Keep snapshots newer than the given duration (e.g., "1y5m7d2h").
    pub fn keep_within(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within", value)
    }

    /// Sets the `--keep-within-hourly` flag.
    pub fn keep_within_hourly(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within-hourly", value)
    }

    /// Sets the `--keep-within-daily` flag.
    pub fn keep_within_daily(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within-daily", value)
    }

    /// Sets the `--keep-within-weekly` flag.
    pub fn keep_within_weekly(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within-weekly", value)
    }

    /// Sets the `--keep-within-monthly` flag.
    pub fn keep_within_monthly(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within-monthly", value)
    }

    /// Sets the `--keep-within-yearly` flag.
    pub fn keep_within_yearly(self, value: &str) -> Self {
        self.with_flag_and_value("keep-within-yearly", value)
    }

    /// Sets the `--keep-tag` flag.
    ///
    /// Keep snapshots with the given taglist.
    pub fn keep_tag(self, value: &str) -> Self {
        self.with_flag_and_value("keep-tag", value)
    }

    /// Sets the `--unsafe-allow-remove-all` flag.
    ///
    /// Allow deleting all snapshots of a snapshot group.
    pub fn unsafe_allow_remove_all(self) -> Self {
        self.with_flag("unsafe-allow-remove-all")
    }

    /// Sets the `--host` flag.
    ///
    /// Only consider snapshots for this host.
    pub fn host(self, value: &str) -> Self {
        self.with_flag_and_value("host", value)
    }

    /// Sets the `--tag` flag.
    ///
    /// Only consider snapshots including the given tags.
    pub fn tag(self, value: &str) -> Self {
        self.with_flag_and_value("tag", value)
    }

    /// Sets the `--path` flag.
    ///
    /// Only consider snapshots including this (absolute) path.
    pub fn path(self, value: &str) -> Self {
        self.with_flag_and_value("path", value)
    }

    /// Sets the `--compact` flag.
    ///
    /// Use compact output format.
    pub fn compact(self) -> Self {
        self.with_flag("compact")
    }

    /// Sets the `--dry-run` flag.
    ///
    /// Do not delete anything, just print what would be done.
    pub fn dry_run(self) -> Self {
        self.with_flag("dry-run")
    }

    /// Sets the `--prune` flag.
    ///
    /// Automatically run the 'prune' command if snapshots have been removed.
    pub fn prune(self) -> Self {
        self.with_flag("prune")
    }

    /// Sets the `--max-unused` flag.
    ///
    /// Tolerate given limit of unused data (e.g., "5%", "10G", "unlimited").
    pub fn max_unused(self, value: &str) -> Self {
        self.with_flag_and_value("max-unused", value)
    }

    /// Sets the `--max-repack-size` flag.
    ///
    /// Stop after repacking this much data in total.
    pub fn max_repack_size(self, value: &str) -> Self {
        self.with_flag_and_value("max-repack-size", value)
    }

    /// Sets the `--repack-cacheable-only` flag.
    ///
    /// Only repack packs which are cacheable.
    pub fn repack_cacheable_only(self) -> Self {
        self.with_flag("repack-cacheable-only")
    }

    /// Sets the `--repack-small` flag.
    ///
    /// Repack pack files below 80% of target pack size.
    pub fn repack_small(self) -> Self {
        self.with_flag("repack-small")
    }

    /// Sets the `--repack-uncompressed` flag.
    ///
    /// Repack all uncompressed data.
    pub fn repack_uncompressed(self) -> Self {
        self.with_flag("repack-uncompressed")
    }

    /// Sets the `--repack-smaller-than` flag.
    ///
    /// Pack below-limit packfiles (e.g., "10M").
    pub fn repack_smaller_than(self, value: &str) -> Self {
        self.with_flag_and_value("repack-smaller-than", value)
    }

    /// Adds a flag without a value.
    pub fn with_flag(mut self, name: &str) -> Self {
        self.builder = self.builder.with_flag(name);
        self
    }

    /// Adds a flag with a value.
    pub fn with_flag_and_value<V: BuilderValue>(mut self, name: &str, value: V) -> Self {
        self.builder = self.builder.with_flag_and_value(name, value);
        self
    }
}
