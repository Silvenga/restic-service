use crate::parsing::ResticMessage;
use crate::{Restic, ResticError};

impl Restic {
    pub async fn get_version(&self) -> Result<ResticVersion, ResticError> {
        let result = self.exec(["version", "--json"]).await?;

        let [version] = result.messages.as_slice() else {
            panic!("expected single element");
        };

        if let ResticMessage::Version {
            version,
            go_version,
            go_os,
            go_arch,
        } = version
        {
            Ok(ResticVersion {
                version: version.clone(),
                go_version: go_version.clone(),
                go_os: go_os.clone(),
                go_arch: go_arch.clone(),
            })
        } else {
            panic!("expected single element");
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResticVersion {
    pub version: String,
    pub go_version: String,
    pub go_os: String,
    pub go_arch: String,
}
