// this file describes the program/app
// and the arguments types and how they are used

extern crate clap;
use self::clap::{Arg, App};
use std::env;
use args::Args;

pub struct Params {
    user_env: Option<String>,
    password_env: Option<String>,
    host_env: Option<String>,
}

impl Params {

    pub fn new<'s>() -> Params {
        Params {
            user_env: env::var("JIRA_USER").ok(),
            password_env: env::var("JIRA_PASSWORD").ok(),
            host_env: env::var("JIRA_HOST").ok(),
        }
    }

    fn get_app(&self) -> App {
        App::new("TIJira")
            .version("0.1.0")
            .author("Scott Hardy <hardy613+TIJira@gmail.com>")
            .about("Terminal Interface for Jira")
            .arg(self.user_arg())
            .arg(self.password_arg())
            .arg(self.host_arg())
            .arg(Arg::with_name("identifier")
                 .short("i")
                 .long("identifier")
                 .help("Load tickets from a Jira project identifier. \
                 Required unless a ticket is provided")
                 .takes_value(true)
                 .required_unless("ticket")
                 .display_order(1))
            .arg(Arg::with_name("ticket")
                 .conflicts_with("identifier")
                 .short("t")
                 .long("ticket")
                 .help("Load a single Jira ticket. \
                    Cannot be used with identifier")
                 .takes_value(true)
                 .display_order(3))
            .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
    }


    fn user_arg(&self) -> Arg {
        let arg = Arg::with_name("user")
             .short("u")
             .long("user")
             .takes_value(true)
             .help("Your Jira username. \
                   Defaults to JIRA_USER environment variable{n}");

        match self.user_env.as_ref() {
            Some(user) => arg.default_value(user),
            None => arg.required(true)
        }
    }

    fn password_arg(&self) -> Arg {
        let arg = Arg::with_name("password")
            .short("p")
            .long("password")
            .takes_value(true)
            .hidden(true);
            //.help("Jira password. \
            //  Defaults to JIRA_PASSWORD environment variable{n}");

        match self.password_env.as_ref() {
            Some(password) => arg.default_value(password),
            None => arg.required(true)
        }
    }
    
    fn host_arg(&self) -> Arg {
        let arg = Arg::with_name("host")
            .short("h")
            .long("host")
            .takes_value(true)
            .help("Jira host URL. \
                  Defaults to JIRA_HOST environment variable{n}");

        match self.host_env.as_ref() {
            Some(host) => arg.default_value(host),
            None => arg.required(true)
        }
    }

    pub fn get_arguments(&self) -> Args {
        let app = self.get_app();
        let matches = app.get_matches_from(env::args_os());
        let from_key = |s: &str| matches.value_of(s).unwrap_or("").to_owned();
        Args {
            user: from_key("user"),
            password: from_key("password"),
            host: from_key("host"),
            identifier: from_key("identifier"),
            ticket: from_key("ticket"),
            api: "/rest/api/2".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
