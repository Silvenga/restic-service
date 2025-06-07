use crate::ResticError;

pub trait IntoResticError<TOk, TErr> {
    fn or_restic_processing_error(self) -> Result<TOk, ResticError>;
}

impl<TOk, TErr: std::error::Error + 'static> IntoResticError<TOk, TErr> for Result<TOk, TErr> {
    fn or_restic_processing_error(self) -> Result<TOk, ResticError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(ResticError::ErrorDuringProcessing(Box::new(err))),
        }
    }
}
