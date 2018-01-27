extern crate serde_json;
use self::serde_json::Value;

pub fn display_ticket(ticket: &Value) {
    let t_fields = &ticket["fields"];
    let t_key = &ticket["key"].as_str().unwrap();
    let t_summary = &t_fields["summary"].as_str().unwrap();
    let t_desc = &t_fields["description"].as_str().unwrap();
    println!("\n[{}] {}\n\nDescription:\n{}",
             t_key, t_summary, t_desc);
}
