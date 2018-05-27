#[macro_use] extern crate quicli;

extern crate reqwest;

use quicli::prelude::*;
use std::process::Command;
use reqwest::header::{Authorization, Basic, ContentType};
use std::env;

/// Generates git branch names based on JIRA Issues
#[derive(Debug, StructOpt)]
struct Cli {
  /// Jira issue number
  issue: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Error {
  error_messages: Vec<String>
}

#[derive(Deserialize)]
struct Success {
  fields: Fields
}

#[derive(Deserialize)]
struct Fields {
  issuetype: IssueType,
  summary: String
}

#[derive(Deserialize)]
struct IssueType {
  name: String
}

main!(|args: Cli| {
  let git_host = Command::new("git")
    .arg("config")
    .arg("git.jira.host")
    .output()
    .expect("failed to execute process");

  let jira_url = format!("https://{}/rest/api/2/issue/{}?fields=summary,issuetype", String::from_utf8_lossy(&git_host.stdout).trim(), args.issue);

  let credentials = Basic {
    username: read_env_var("JIRA_USERNAME"),
    password: Some(read_env_var("JIRA_PASSWORD")),
  };

  let http_client = reqwest::Client::new();
  let mut resp = http_client.get(&jira_url).header(Authorization(credentials)).header(ContentType::json()).send()?;

  if resp.status().is_success() {
    let success: Success = resp.json()?;
    let issue_type = str::replace(&success.fields.issuetype.name.to_lowercase(), " ", "_");
    let issue_summary = str::replace(&success.fields.summary.to_lowercase(), " ", "_");
    let branch_name = format!("{}/{}-{}", issue_type, args.issue, issue_summary);

    Command::new("git")
      .arg("checkout")
      .arg("-b")
      .arg(&branch_name)
      .output()
      .expect("failed to execute process");

    println!("Success: {:?}", branch_name);
  } else {
    let err: Error = resp.json()?;
    println!("Error: {:?}", err.error_messages);
  }
});

fn read_env_var(env_var: &str) -> String {
  return match env::var(env_var) {
    Ok(r) => r,
    Err(e) => {
      eprintln!("Couldn't read {} ({})", env_var, e);
      ::std::process::exit(1);
    }
  };
}
