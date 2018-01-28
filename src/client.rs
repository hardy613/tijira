
use reqwest::{Client};

trait IssueClient {
    fn fetch_issues() -> Result<Vec<Issue>, Error>;
    fn assign_issue(&issue: Issue) -> Result<(), Error>;
}

