use serde::Serialize;

#[derive(Serialize)]
pub struct ValidateResponse {
    pub success: bool,
}
