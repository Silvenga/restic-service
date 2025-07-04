use crate::errors::ResticError;
use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::messages::{Initialized, ResticInitMessage};
use crate::{CommandBuilder, Restic};

impl Restic {
    /// Initializes a new Restic repository using the current config.
    pub async fn init(&self) -> Result<Initialized, ResticError> {
        let mut messages = Vec::new();

        let arguments = CommandBuilder::new().with_verb("init").build();

        self.exec_json(arguments, |message: ResticInitMessage| {
            messages.push(message);
        })
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        match Initialized::try_from(message) {
            Ok(result) => Ok(result),
            Err(error) => Err(ResticError::UnexpectedResponse(error)),
        }
    }

    /// Initializes a new Restic repository if it does not already exist.
    pub async fn init_if_not_exists(&self) -> Result<Option<Initialized>, ResticError> {
        match self.can_open().await? {
            true => Ok(None),
            false => self.init().await.map(Some),
        }
    }
}
