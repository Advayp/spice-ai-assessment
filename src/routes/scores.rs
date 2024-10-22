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
struct ScoreFields {
    name: String,
    uid: String,
    score: u64,
    notes: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddScoreRequest {
    scores: Vec<ScoreFields>,
}

#[post("/scores/insert", data = "<scores>")]
pub async fn add_scores(scores: Json<AddScoreRequest>) -> Json<ValidateResponse> {
    let all_scores = scores.into_inner().scores;

    let mut converted_scores: Vec<ScoreInfo> = vec![];

    for score in all_scores {
        converted_scores.push(ScoreInfo {
            name: score.name.clone(),
            uid: score.uid.clone(),
            score: score.score.clone(),
            notes: score.notes.clone(),
            ..Default::default()
        });
    }

    insert_rows(&converted_scores).await.unwrap();

    Json(ValidateResponse { success: true })
}
