use crate::models::scores::ScoreInfo;

use super::utils::{build_request_client, build_rest_url, RequestTypes};

pub async fn get_all_scores() -> Result<Vec<ScoreInfo>, Box<dyn std::error::Error>> {
    let url = build_rest_url("/scores?select=*".to_string());
    let res = build_request_client(&url, RequestTypes::GET).send().await?;

    let body = res.json::<Vec<ScoreInfo>>().await?;

    Ok(body)
}
