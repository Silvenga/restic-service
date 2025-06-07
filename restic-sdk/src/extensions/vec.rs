use thiserror::Error;

pub trait VecHelpers<T> {
    fn into_single(self) -> Result<T, SingleItemError>;
}

impl<T> VecHelpers<T> for Vec<T> {
    fn into_single(mut self) -> Result<T, SingleItemError> {
        match self.len() {
            0 => Err(SingleItemError::Empty),
            1 => Ok(self.pop().unwrap()),
            n => Err(SingleItemError::TooMany(n)),
        }
    }
}

#[derive(Debug, Error)]
pub enum SingleItemError {
    #[error("expected a single item, but got none")]
    Empty,
    #[error("expected a single item, but got {0} items")]
    TooMany(usize),
}
