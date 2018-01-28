# TiJira
Terminal Interface for Jira

## Help
From the `--help` command.

```
TiJira 0.1.0
Scott Hardy <hardy613+TiJira@gmail.com>
Terminal Interface for Jira

USAGE:
    tijira [FLAGS] [OPTIONS] --identifier <identifier>

FLAGS:
        --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -i, --identifier <identifier>     Load tickets from a Jira project identifier.
                                      Required unless a ticket is provided
    -t, --ticket <ticket>             Load a single Jira ticket. Cannot be used with identifier
    -h, --host <host>                 Jira host URL. Defaults to JIRA_HOST environment variable
    -u, --user <user>                 Your Jira username. Defaults to JIRA_USER environment variable
    -p, --password <spassword>        Your Jira password. Defaults to JIRA_PASSWORD environment variable
```
