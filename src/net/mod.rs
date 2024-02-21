pub mod models;

use reqwest::header::HeaderMap;

pub async fn fetch_data(
    host: String,
    api_key: String,
) -> Result<models::Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    _ = headers.insert("X-Redmine-API-Key", api_key.parse().unwrap());

    let host = host + "/issues.json?assigned_to_id=me";
    let response = client.get(host).headers(headers).send().await?;

    let data: models::Response = response.error_for_status()?.json().await?;
    Ok(data)
}
