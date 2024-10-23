use reqwest::Response;
use rocket::serde::{Deserialize, Serialize};

use super::utils::{build_auth_url, build_request_client, RequestTypes};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(
    email: String,
    password: String,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = build_auth_url("signup".to_string());

    let req = build_request_client(&url, RequestTypes::POST);

    let data = AuthData { email, password };

    let res = req.json(&data).send().await?;

    Ok(res)
}

pub async fn login(
    email: String,
    password: String,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = build_auth_url("/token?grant_type=password".to_string());
    let req = build_request_client(&url, RequestTypes::POST);

    let data = AuthData { email, password };

    let res = req.json(&data).send().await?;

    Ok(res)
}
