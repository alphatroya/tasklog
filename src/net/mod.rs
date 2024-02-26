pub mod models;

use chrono::Utc;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

pub async fn fetch_data(
    host: String,
    api_key: String,
) -> Result<models::Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let yesterday = Utc::now() - chrono::Duration::days(1);
    let yesterday = yesterday.format(">=%Y-%m-%d").to_string();

    let mut query_params = HashMap::new();
    query_params.insert("assigned_to_id", "me");
    query_params.insert("updated_on", &yesterday);

    let mut headers = HeaderMap::new();
    _ = headers.insert("X-Redmine-API-Key", api_key.parse().unwrap());

    let host = host + "/issues.json";
    let response = client
        .get(host)
        .query(&query_params)
        .headers(headers)
        .send()
        .await?;

    let data: models::Response = response.error_for_status()?.json().await?;
    Ok(data)
}
