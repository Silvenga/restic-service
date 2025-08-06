use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppApiError {
    #[error("job not found")]
    JobNotFound,
    #[error("internal server error")]
    InternalServerError,
}

impl error::ResponseError for AppApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppApiError::JobNotFound => StatusCode::NOT_FOUND,
            AppApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(
                r#"{{"message":"{self}", code:"{code}"}}"#,
                code = self.status_code()
            ))
    }
}
