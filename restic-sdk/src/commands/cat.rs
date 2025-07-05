use crate::commands::exec::MessageOutputType;
use crate::errors::ResticError;
use crate::{ArgumentsBuilder, Restic};

impl Restic {
    /// Retrieves the content of a specific path in the Restic repository.
    /// The whole file will be buffered into memory, so it is not suitable for large files.
    pub async fn cat(&self, path: &str) -> Result<String, ResticError> {
        let mut output = String::new();
        self.exec(
            ArgumentsBuilder::new().with_verb("cat").with_value(path),
            |message, output_type| {
                if output_type == MessageOutputType::Stdout {
                    output.push_str(&message)
                }
            },
        )
        .await?;

        Ok(output)
    }

    /// Checks if the Restic repository exists and can be opened using the configured password.
    pub async fn can_open(&self) -> Result<bool, ResticError> {
        match self.cat("config").await {
            Ok(_) => Ok(true),
            Err(ResticError::RepositoryDoesNotExist) => Ok(false),
            Err(err) => Err(err),
        }
    }
}
