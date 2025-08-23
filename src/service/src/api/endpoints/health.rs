use actix_web::{get, web};
use serde::{Deserialize, Serialize};

#[get("/health")]
async fn health() -> web::Json<HealthResponse> {
    web::Json(HealthResponse { ok: true })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HealthResponse {
    ok: bool,
}
