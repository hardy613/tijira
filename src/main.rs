#[macro_use] extern crate log;

mod args;
mod params;
mod query;
mod jira;

use params::Params;
use jira::Jira;

fn main() {
    let params = Params::new();
    let args  = params.get_arguments();
    info!("arguments: {:?}", args);
    // from the mod params,  indentifier is required
    // and if ticket is set then indentifier cannot be used
    let query = match args.identifier.is_empty() {
        false => query::build_identifier_request(&args),
        true => query::build_ticket_request(&args)
    };
    debug!("query: {:?}", query);
    println!("query: {:?}", query);

    let jira = Jira::new();
    let issues = jira.send_request(&query, &args);
    println!("result {:?}", issues);
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
