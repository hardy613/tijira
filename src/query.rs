// this file builds the user request
use params::Args;

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
    query.push_str("key=");
    query.push_str(&args.ticket);
    query.push_str("&fields=*all\
    &maxResults=1");
    query
}

