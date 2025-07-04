use crate::errors::ResticError;
use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::messages::{ResticVersionMessage, Version};
use crate::{CommandBuilder, Restic};

impl Restic {
    pub async fn cat(&self, path: &str) -> Result<Version, ResticError> {
        let mut messages = Vec::new();

        let arguments = CommandBuilder::new()
            .with_verb("cat")
            .with_value(path)
            .build();

        self.exec_json(arguments, |message: ResticVersionMessage| {
            messages.push(message);
        })
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        match Version::try_from(message) {
            Ok(result) => Ok(result),
            Err(error) => Err(ResticError::UnexpectedResponse(error)),
        }
    }

    pub async fn cat_json(&self, path: &str) -> Result<ResticVersionMessage, ResticError> {
        let mut messages = Vec::new();

        self.exec_json(
            vec!["check".to_string(), path.to_string()],
            |message: ResticVersionMessage| {
                messages.push(message);
            },
        )
        .await?;

        messages.into_single().or_restic_processing_error()
    }
}
