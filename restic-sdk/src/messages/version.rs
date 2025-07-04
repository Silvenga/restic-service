use crate::messages::ExitError;
use crate::restic_message;
use serde::Deserialize;

restic_message! {
    pub enum ResticVersionMessage {
        #[serde(rename = "version")]
        Version,
    }
}

/// The version of restic
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Version {
    /// restic version
    pub version: String,
    /// Go compile version
    pub go_version: String,
    /// Go OS
    pub go_os: String,
    /// Go architecture
    pub go_arch: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "version",
            "version": "0.12.0",
            "go_version": "go1.20.3",
            "go_os": "linux",
            "go_arch": "amd64"
        }"#;
        let message = ResticVersionMessage::parse_message(json).expect("parse should succeed");

        let result = Version::try_from(message).expect("should convert");
        assert_eq!(result.version, "0.12.0");
        assert_eq!(result.go_version, "go1.20.3");
        assert_eq!(result.go_os, "linux");
        assert_eq!(result.go_arch, "amd64");
    }
}
