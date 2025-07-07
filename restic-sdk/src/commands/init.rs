use crate::errors::ResticError;
use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::messages::{Initialized, ResticInitMessage};
use crate::{ArgumentsBuilder, Restic};
use tokio_util::sync::CancellationToken;

impl Restic {
    /// Initializes a new Restic repository using the current config.
    pub async fn init(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Initialized, ResticError> {
        let mut messages = Vec::new();
        self.exec_json(
            ArgumentsBuilder::new().with_verb("init"),
            |message: ResticInitMessage| {
                messages.push(message);
            },
            cancellation_token,
        )
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        match Initialized::try_from(message) {
            Ok(result) => Ok(result),
            Err(error) => Err(ResticError::UnexpectedResponse(error)),
        }
    }

    /// Initializes a new Restic repository if it does not already exist.
    pub async fn init_if_not_exists(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Option<Initialized>, ResticError> {
        match self.can_open(cancellation_token).await? {
            true => Ok(None),
            false => self.init(cancellation_token).await.map(Some),
        }
    }
}
