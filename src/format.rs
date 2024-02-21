use crate::net::models;
use chrono::{DateTime, Datelike, Local, Utc};

pub fn print(issues: Vec<models::Issues>, host: String) {
    println!("");
    let now = Local::now();
    println!(
        "- **{}** #[[Heads and Hands]] #Redmine",
        now.format("%H:%M").to_string()
    );
    let pad = " ".repeat(4);
    for issue in issues {
        println!(
            "{} - [{}]({}/issues/{})",
            pad, issue.subject, host, issue.id
        );
        println!("{}   redmine_id:: `{}`", pad, issue.id);
        println!("{}   redmine_project:: [[{}]]", pad, issue.project.name);
        println!("{}   redmine_status:: {}", pad, issue.status.name);
        println!("{}   redmine_tracker:: {}", pad, issue.tracker.name);
        println!("{}   redmine_author:: [[{}]]", pad, issue.author.name);
        println!(
            "{}   redmine_created:: [[{}]]",
            pad,
            format_date(issue.created_on)
        );
        println!(
            "{}   redmine_updated:: [[{}]]",
            pad,
            format_date(issue.updated_on)
        );
    }
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
