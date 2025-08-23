use serde::Deserialize;

/// A backup verbose status message from restic, includes details about backed up files.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BackupVerboseStatus {
    /// Either "new", "unchanged", "modified" or "scan_finished"
    pub action: BackupVerboseStatusAction,
    /// The item in question
    pub item: String,
    /// How long it took, in seconds
    #[serde(rename = "duration")]
    pub duration_seconds: f64,
    /// How big the item is
    pub data_size: u64,
    /// How big the item is in the repository
    pub data_size_in_repo: u64,
    /// How big the metadata is
    pub metadata_size: u64,
    /// How big the metadata is in the repository
    pub metadata_size_in_repo: u64,
    /// Total number of files
    pub total_files: u64,
}

#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum BackupVerboseStatusAction {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "unchanged")]
    Unchanged,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "scan_finished")]
    ScanFinished,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::ResticBackupMessage;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "verbose_status",
            "action": "new",
            "item": "example.txt",
            "duration": 0.123,
            "data_size": 1024,
            "data_size_in_repo": 512,
            "metadata_size": 256,
            "metadata_size_in_repo": 128,
            "total_files": 1
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupVerboseStatus::try_from(message).expect("should convert");

        assert_eq!(result.action, BackupVerboseStatusAction::New);
        assert_eq!(result.item, "example.txt");
        assert_eq!(result.duration_seconds, 0.123);
        assert_eq!(result.data_size, 1024);
        assert_eq!(result.data_size_in_repo, 512);
        assert_eq!(result.metadata_size, 256);
        assert_eq!(result.metadata_size_in_repo, 128);
        assert_eq!(result.total_files, 1);
    }
}
