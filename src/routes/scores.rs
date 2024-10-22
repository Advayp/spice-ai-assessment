use crate::{
    database::scores::{get_all_scores, insert_rows},
    models::scores::ScoreInfo,
    types::ValidateResponse,
};
use serde::Serialize;

use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Serialize)]
pub struct ScoreResponse {
    scores: Vec<ScoreInfo>,
}

#[get("/scores/all")]
pub async fn fetch_all_scores() -> Json<ScoreResponse> {
    let scores = get_all_scores().await.unwrap();

    let res = ScoreResponse { scores };

    Json(res)
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddScoreRequest {
    scores: Vec<ScoreInfo>,
}

#[post("/scores/insert", data = "<scores>")]
pub async fn add_scores(scores: Json<AddScoreRequest>) -> Json<ValidateResponse> {
    let all_scores = scores.into_inner().scores;

    insert_rows(&all_scores).await.unwrap();

    Json(ValidateResponse { success: true })
}
