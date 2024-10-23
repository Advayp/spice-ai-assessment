use rocket::serde::{Deserialize, Serialize};

use crate::models::scores::ScoreInfo;

#[derive(Deserialize, Serialize)]
struct SearchMatch {
    value: String,
    score: f64,
    dataset: String,
    metadata: ScoreInfo,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    matches: Vec<SearchMatch>,
    duration: f64,
}

#[derive(Serialize)]
struct SearchRequest {
    datasets: Vec<String>,
    text: String,
    additional_columns: Vec<String>,
    limit: u16,
}

fn build_body(text: String, limit: u16) -> SearchRequest {
    SearchRequest {
        datasets: vec!["score".to_string()],
        text,
        additional_columns: vec![
            "id".to_string(),
            "created_at".to_string(),
            "name".to_string(),
            "score".to_string(),
            "notes".to_string(),
            "uid".to_string(),
        ],
        limit,
    }
}

pub async fn search(text: String, limit: u16) -> SearchResponse {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:8090/v1/search")
        .json(&build_body(text, limit))
        .send()
        .await
        .unwrap();

    res.json::<SearchResponse>().await.unwrap()
}
