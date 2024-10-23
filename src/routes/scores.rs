use crate::{
    ai::{
        embeddings::search,
        open_ai::{get_client, make_request},
    },
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

#[derive(Deserialize, Debug, Clone)]
pub struct QueryAIRequest {
    messages: Vec<String>,
    uid: String,
}

#[derive(Serialize, Debug)]
pub struct QueryAIResponse {
    response: String,
}

#[get("/scores/query", data = "<messages>")]
pub async fn query_ai(messages: Json<QueryAIRequest>) -> Json<QueryAIResponse> {
    let mut requested_prompts = messages.clone().into_inner().messages;
    let uid = messages.into_inner().uid;

    let client = get_client();

    requested_prompts.push(format!(
        "Filter your results to only users with \"uid\" equal to {}",
        uid
    ));

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

#[derive(Deserialize, Clone)]
pub struct VectorSearchRequest {
    text: String,
    limit: u16,
    uid: String,
}

#[get("/scores/vsearch", data = "<req>")]
pub async fn vectorized_search(req: Json<VectorSearchRequest>) -> Json<Vec<ScoreInfo>> {
    let search_text = req.clone().into_inner().text;
    let limit = req.clone().into_inner().limit;
    let uid = req.into_inner().uid;

    let search_results = search(search_text, limit).await;

    let mut res: Vec<ScoreInfo> = vec![];

    for valid_match in search_results.matches {
        if valid_match.metadata.uid == uid {
            res.push(valid_match.metadata);
        }
    }

    Json(res)
}
