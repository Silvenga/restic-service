use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// A backup error message from restic (e.g. a file disappeared during backup)
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct BackupError {
    /// Error message
    pub error: Error,
    // What restic was trying to do
    pub during: String,
    /// Usually, the path of the problematic file
    pub item: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
}

impl Display for BackupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{message}' on '{item}' (during {during})",
            message = self.error.message,
            item = self.item,
            during = self.during
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::ResticBackupMessage;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "error",
            "error": {
                "message": "Failed to open file"
            },
            "during": "backup",
            "item": "/path/to/file.txt"
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupError::try_from(message).expect("should convert");
        assert_eq!(result.error.message, "Failed to open file");
        assert_eq!(result.during, "backup");
        assert_eq!(result.item, "/path/to/file.txt");
    }
}
