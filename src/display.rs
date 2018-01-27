extern crate serde_json;
use self::serde_json::Value;

pub fn display_ticket(ticket: &Value) {
    let fields = &ticket["fields"];
    let key = &ticket["key"].as_str().unwrap();
    let summary = &fields["summary"].as_str().unwrap();
    let desc = &fields["description"].as_str().unwrap();
    println!("{}\n\n{}\n\n{}\n\n",
             key, summary, desc);
}
