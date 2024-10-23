use crate::{
    ai::open_ai::{get_client, make_request},
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

#[derive(Deserialize)]
pub struct ScoreRequest {
    uid: String,
}

#[get("/scores/all", data = "<uid>")]
pub async fn fetch_all_scores(uid: Json<ScoreRequest>) -> Json<ScoreResponse> {
    let scores = get_all_scores(uid.into_inner().uid).await.unwrap();

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

#[derive(Deserialize, Debug)]
pub struct QueryAIRequest {
    messages: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct QueryAIResponse {
    response: String,
}

#[get("/scores/query", data = "<messages>")]
pub async fn query_ai(messages: Json<QueryAIRequest>) -> Json<QueryAIResponse> {
    let requested_prompts = messages.into_inner().messages;

    let client = get_client();

    let response = make_request(client, requested_prompts).await;

    let relevant_response = &response.choices[0];

    match &relevant_response.message.content {
        Some(message) => Json(QueryAIResponse {
            response: message.to_string(),
        }),
        None => Json(QueryAIResponse {
            response: String::from("No Response"),
        }),
    }
}
