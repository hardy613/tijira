// creates a request client incase the user makes multiple 
// reqwests in a single useage
extern crate serde_json;
use reqwest::{Client, Response, StatusCode};
use reqwest::header::ContentType;
use params::Args;
use std::io::Read;
use self::serde_json::{Value, Error};
use client::IssueClient;

pub struct Jira {
    pub client: Client,
}

impl IssueClient for Jira {
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

    fn parse_response(mut response: Response) -> String {
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

    pub fn parse_issues(issues: &String) -> Result<Value, Error> {
        let items: Value = self::serde_json::from_str(&issues)?;
        Ok(items)
    }
}
