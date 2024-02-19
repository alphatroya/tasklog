use chrono::{DateTime, Datelike, Utc};
use std::env;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("REDMINE_API_KEY")?;
    let host = env::var("REDMINE_HOST")?;
    let data = fetch_data(host.clone(), api_key).await;
    format(
        if let Ok(response) = data {
            response.issues
        } else {
            vec![]
        },
        host,
    );
    Ok(())
}

fn format(issues: Vec<Issues>, host: String) {
    println!("");
    for issue in issues {
        println!("- [{}]({}/issues/{})", issue.subject, host, issue.id);
        println!("  redmine_id:: `{}`", issue.id);
        println!("  redmine_project:: [[{}]]", issue.project.name);
        println!("  redmine_status:: {}", issue.status.name);
        println!("  redmine_tracker:: {}", issue.tracker.name);
        println!("  redmine_author:: [[{}]]", issue.author.name);
        println!("  redmine_created:: [[{}]]", format_date(issue.created_on));
        println!("  redmine_updated:: [[{}]]", format_date(issue.updated_on));
    }
}

async fn fetch_data(host: String, api_key: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    _ = headers.insert("X-Redmine-API-Key", api_key.parse().unwrap());

    let host = host + "/issues.json?assigned_to_id=me";
    let response = client.get(host).headers(headers).send().await?;

    let data: Response = response.error_for_status()?.json().await?;
    Ok(data)
}

fn format_date(date: DateTime<Utc>) -> String {
    let day = date.day();
    let day_suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };

    let month = date.format("%b").to_string();
    let year = date.format("%Y").to_string();

    format!("{} {}{}, {}", month, day, day_suffix, year)
}

use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    issues: Vec<Issues>,
}

#[derive(Debug, Deserialize)]
struct Issues {
    id: i32,
    subject: String,
    project: Project,
    status: Status,
    tracker: Tracker,
    author: Author,
    created_on: DateTime<Utc>,
    updated_on: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct Tracker {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Status {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
}
