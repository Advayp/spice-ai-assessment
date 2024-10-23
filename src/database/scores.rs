use serde::Serialize;

use crate::models::scores::ScoreInfo;

use super::utils::{build_request_client, build_rest_url, RequestTypes};

pub async fn get_all_scores(uid: String) -> Result<Vec<ScoreInfo>, Box<dyn std::error::Error>> {
    let url = build_rest_url(format!("/scores?uid=eq.{}&select=*", uid));
    let res = build_request_client(&url, RequestTypes::GET).send().await?;

    let body = res.json::<Vec<ScoreInfo>>().await?;

    Ok(body)
}

#[derive(Serialize)]
struct InsertRowRequest {
    name: String,
    score: u64,
    notes: String,
    uid: String,
}

pub async fn insert_rows(items: &Vec<ScoreInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_rest_url("/scores".to_string());
    let request_builder = build_request_client(&url, RequestTypes::POST);

    let mut req: Vec<InsertRowRequest> = vec![];

    for item in items {
        req.push(InsertRowRequest {
            name: item.name.clone(),
            score: item.score.clone(),
            notes: item.notes.clone(),
            uid: item.uid.clone(),
        });
    }

    request_builder.json(&req).send().await?;

    Ok(())
}
