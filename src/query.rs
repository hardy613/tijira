// this file builds the user request
use args::Args;

pub fn build_identifier_request(args: &Args) -> String {
    let mut query = String::new();
    query.push_str("identifer");
    println!("1 {:?}", args);
    query
}

pub fn build_ticket_request(args: &Args) -> String {
    let mut query = String::new();
    query.push_str(&args.host);
    query.push_str(&args.api);
    query.push_str("/issue/");
    query.push_str(&args.ticket);
    query
}

