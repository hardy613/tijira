extern crate futures;
extern crate reqwest;
use self::reqwest::{Client, Response, StatusCode, Error};
use self::reqwest::header::ContentType;
use args::Args;
use std::io::Read;

pub struct Jira {
    pub results: Vec<String>,
    pub client: Client,
}

impl Jira {
    pub fn new() -> Jira {
        Jira {
            results: Vec::new(),
            client: Client::new()
        }
    }

    pub fn send_request(&self, url: &str, args: &Args) -> Option<Response> {
        let user: &str = &args.user;
        let password: &str = &args.password;
        let client = &self.client;
        let mut request = client.get(&url.to_owned());
        request.basic_auth(user, Some(password));
        request.header(ContentType::json());
        let result = request.send();
        match result {
            Ok(mut response) => &self.parse_response(Some(response)),
            Err(error) => {
                println!("Error: {}", error);
            }
        }

    }

    fn parse_response(&self, response: Option<Response>)  {

        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                println!("Body: {:?}", body);
            },
            code => {
                let mut text = String::new();
                response.read_to_string(&mut text).unwrap();
                println!("Error: {:?}", code);
                println!("response: {:?}", text);
            }


        }
    }
}
