use chrono::{DateTime, Utc};
use serde::Deserialize;

type UtcDateTime = DateTime<Utc>;

/// Summary of a successful backup operation
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BackupSummary {
    /// Whether the backup was a dry run
    pub dry_run: bool,
    /// Number of new files
    pub files_new: u64,
    /// Number of files that changed
    pub files_changed: u64,
    /// Number of files that did not change
    pub files_unmodified: u64,
    /// Number of new directories
    pub dirs_new: u64,
    /// Number of directories that changed
    pub dirs_changed: u64,
    /// Number of directories that did not change
    pub dirs_unmodified: u64,
    /// Number of data blobs added
    pub data_blobs: i64,
    /// Number of tree blobs added
    pub tree_blobs: i64,
    /// Amount of (uncompressed) data added, in bytes
    pub data_added: u64,
    /// Amount of data added (after compression), in bytes
    pub data_added_packed: u64,
    /// Total number of files processed
    pub total_files_processed: u64,
    /// Total number of bytes processed
    pub total_bytes_processed: u64,
    /// Time at which the backup was started
    pub backup_start: UtcDateTime,
    /// Time at which the backup was completed
    pub backup_end: UtcDateTime,
    /// Total time it took for the operation to complete
    pub total_duration: f64,
    /// ID of the new snapshot. Field is omitted if snapshot creation was skipped
    pub snapshot_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::ResticBackupMessage;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "summary",
            "dry_run": false,
            "files_new": 10,
            "files_changed": 5,
            "files_unmodified": 15,
            "dirs_new": 2,
            "dirs_changed": 1,
            "dirs_unmodified": 3,
            "data_blobs": 20,
            "tree_blobs": 5,
            "data_added": 102400,
            "data_added_packed": 51200,
            "total_files_processed": 30,
            "total_bytes_processed": 204800,
            "backup_start": "2023-10-01T12:00:00Z",
            "backup_end": "2023-10-01T12:05:00Z",
            "total_duration": 300.0,
            "snapshot_id": "abc123"
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupSummary::try_from(message).expect("should convert");

        assert!(!result.dry_run);
        assert_eq!(result.files_new, 10);
        assert_eq!(result.files_changed, 5);
        assert_eq!(result.files_unmodified, 15);
        assert_eq!(result.dirs_new, 2);
        assert_eq!(result.dirs_changed, 1);
        assert_eq!(result.dirs_unmodified, 3);
        assert_eq!(result.data_blobs, 20);
        assert_eq!(result.tree_blobs, 5);
        assert_eq!(result.data_added, 102400);
        assert_eq!(result.data_added_packed, 51200);
        assert_eq!(result.total_files_processed, 30);
        assert_eq!(result.total_bytes_processed, 204800);
        assert_eq!(
            result.backup_start,
            DateTime::parse_from_rfc3339("2023-10-01T12:00:00Z").unwrap()
        );
        assert_eq!(
            result.backup_end,
            DateTime::parse_from_rfc3339("2023-10-01T12:05:00Z").unwrap()
        );
        assert_eq!(result.total_duration, 300.0);
        assert_eq!(result.snapshot_id, Some("abc123".to_string()));
    }
}
