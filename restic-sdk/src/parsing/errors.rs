use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Serde errored")]
    SerdeError(#[from] serde_json::Error),
}
