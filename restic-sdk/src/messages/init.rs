use crate::messages::ExitError;
use crate::restic_message;
use serde::Deserialize;

restic_message! {
    pub enum ResticInitMessage {
        #[serde(rename = "initialized")]
        Initialized,
    }
}

/// The version of restic
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Initialized {
    /// ID of the created repository
    pub id: String,
    /// URL of the repository
    pub repository: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "initialized",
            "id": "1234567890abcdef",
            "repository": "s3:http://example.com/repo"
        }"#;
        let message = ResticInitMessage::parse_message(json).expect("parse should succeed");

        let result = Initialized::try_from(message).expect("should convert");
        assert_eq!(result.id, "1234567890abcdef");
        assert_eq!(result.repository, "s3:http://example.com/repo");
    }
}
