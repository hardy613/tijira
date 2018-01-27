// this file describes the arguments 
// tijira will use

#[derive(Debug)]
pub struct Args {
    pub user: String,
    pub password: String,
    pub host: String,
    pub identifier: String,
    pub ticket: String,
    pub api: String
}

impl Args {
    pub fn new() -> Args {
        Args {
            user: String::from(""),
            password: String::from(""),
            host: String::from(""),
            api: String::from(""),
            identifier: String::from(""),
            ticket: String::from(""),
        }
    }
}
