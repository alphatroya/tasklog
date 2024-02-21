use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub issues: Vec<Issues>,
}

#[derive(Debug, Deserialize)]
pub struct Issues {
    pub id: i32,
    pub subject: String,
    pub project: Project,
    pub status: Status,
    pub tracker: Tracker,
    pub author: Author,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Tracker {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
}
