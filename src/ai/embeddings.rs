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
