// creates a request client incase the user makes multiple 
// reqwests in a single useage
extern crate futures;
extern crate reqwest;
use self::reqwest::{Client, Response, StatusCode};
use self::reqwest::header::ContentType;
use params::Args;
use std::io::Read;

pub struct Jira {
    pub client: Client,
}

impl Jira {
    pub fn new() -> Jira {
        Jira {
            client: Client::new()
        }
    }

    pub fn send_request(&self, url: &str, args: &Args) -> String {
        let user: &str = &args.user;
        let password: &str = &args.password;
        let client = &self.client;
        let mut request = client.get(&url.to_owned());
        request.basic_auth(user, Some(password));
        request.header(ContentType::json());
        match request.send() {
            Ok(response) => Jira::parse_response(response),
            Err(error) => {
                println!("Error: {}", error);
                error.to_string()
            }
        }
    }

    fn parse_response(mut response: Response) -> String  {
        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                body           
            },
            code => {
                let mut text = String::new();
                response.read_to_string(&mut text).unwrap();
                println!("Error: {:?}", code);
                text
            }
        }
    }
}
