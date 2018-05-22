#![feature(iterator_for_each)]
#![feature(custom_attribute)]

#[macro_use] extern crate quicli;

extern crate reqwest;

use quicli::prelude::*;
use std::process::Command;

/// Generates git branch names based on JIRA Issues
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,

    /// Jira issue number
    issue: String,

    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Error {
    error_messages: Vec<String>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Success {
    body: String
}

main!(|args: Cli, log_level: verbosity| {
    let git_host = Command::new("git")
                 .arg("config")
                 .arg("git.jira.host")
                 .output()
                 .expect("failed to execute process");

 let jira_url = format!("https://{}/rest/api/2/issue/{}?fields=summary,issuetype", String::from_utf8_lossy(&git_host.stdout).trim(), args.issue);

 let mut resp = reqwest::get(&jira_url)?;

 if resp.status().is_success() {
     let success: Success = resp.json()?;
     println!("Success: {:?}", success.body);
 } else {
     let err: Error = resp.json()?;
     println!("stdout: {:?}", err.error_messages);
 }
});
