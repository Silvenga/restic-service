use thiserror::Error;

pub trait ResticMessage: Sized {
    fn parse_message(message: &str) -> Result<Self, ParseError>;
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Serde errored")]
    SerdeError(#[from] serde_json::Error),
}
