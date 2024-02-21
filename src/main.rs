mod format;
mod net;
use std::env;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("REDMINE_API_KEY")?;
    let host = env::var("REDMINE_HOST")?;
    let data = net::fetch_data(host.clone(), api_key).await;
    format::print(
        if let Ok(response) = data {
            response.issues
        } else {
            vec![]
        },
        host,
    );
    Ok(())
}
