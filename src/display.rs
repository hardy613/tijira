extern crate serde_json;

use self::serde_json::Value;
use termion::{color, style};

pub fn display_ticket<'t>(ticket: &Value) {
    let fields = &ticket["fields"];
    let key = &ticket["key"].as_str().unwrap();
    let status = &fields["status"]["name"].as_str().unwrap();
    let summary = &fields["summary"].as_str().unwrap();
    let desc = &fields["description"].as_str().unwrap();
    let assignee = &fields["assignee"]["displayName"]
        .as_str().unwrap_or("Unassigned");
    let reporter = &fields["reporter"]["displayName"].as_str().unwrap();
    let status_color = match status {
        &"To Do" => color::Rgb(20, 145, 165),
        &"In Progress" => color::Rgb(14, 130, 12),
        &"Blocked" => color::Rgb(150, 3, 3),
        &"Dev Completed" => color::Rgb(63, 84, 0),
        &"Ready for QA" => color::Rgb(55, 0, 255),
        &"Done" => color::Rgb(95, 0, 175),
        _ => color::Rgb(33, 79, 73)
    };

    print!("{}{} |{} {} {}| ",
           style::Bold, key,
           color::Bg(status_color), status, color::Bg(color::Reset));
    print!("Assignee: {} | ", assignee);
    println!("Reporter: {}{}", reporter, style::Reset);
    println!("\n{}", summary);
    println!("\n{}\n", desc);
}
