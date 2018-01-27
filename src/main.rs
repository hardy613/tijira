#[macro_use] extern crate log;
extern crate serde;
extern crate futures;
extern crate reqwest;
extern crate clap;

mod params;
mod query;
mod jira;
mod display;

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
    info!("query: {:?}", query);
    println!("query: {:?}", query);

    let jira = Jira::new();
    let request = jira.send_request(&query, &args);
    info!("result {:?}", request);

    let response = Jira::parse_issues(&request).ok().unwrap();
    info!("response {:?}", response);
    
    let issues = response["issues"].as_array().unwrap();
    if issues.len() == 0 {
        println!("No issue(s) found");
    } else if issues.len() == 1 {
        display::display_ticket(&issues[0]);
    } else {
        println!("{} issues found", issues.len());
    }

}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
