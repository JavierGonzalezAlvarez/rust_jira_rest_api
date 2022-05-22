use chrono::prelude::*;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[tokio::main]
pub async fn jira() -> Result<(), reqwest::Error> {
  let summary: String = String::from("Jira: Issue 2");
  let data = json!(
      {
          "fields": {
          "summary": &summary.to_owned(),
          "issuetype": {
            "id": "10001"
          },
          "project": {
            "key": "DJAN"
          },
          "labels": [
            "Ram",
            "Development"
          ],
          "description": {
            "type": "doc",
            "version": 1,
            "content": [
              {
                "type": "paragraph",
                "content": [
                  {
                    "text": "Data have changed",
                    "type": "text"
                  }
                ]
              }
            ]
          }

        }
      }
  );

  println!("body (json) => {}", data);
  let now = Utc::now();
  let date = now.format("%Y-%m-%d %H:%M:%S").to_string();

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("./logs/response_api_jira.log")
    .unwrap();

  let userjira = std::env::var("USERJIRA").expect("user");
  let token = std::env::var("TOKEN").expect("token");

  let resp = reqwest::Client::new()
    .post("https://jira1414.atlassian.net/rest/api/3/issue")
    .header("Content-Type", "application/json")
    .basic_auth(userjira, Some(token))
    .json(&data)
    .send()
    .await?;
  match resp.error_for_status() {
    Ok(_res) => {
      println!("response with no errors, ticket created");
      writeln!(
        file,
        "log: ok response ticket {} - status: {:#?}",
        &date,
        reqwest::StatusCode::OK
      )
      .unwrap();
    }
    Err(err) => {
      log::debug!("Response: {:#?}", err.status());
      println!(
        "found an error response ticket from jira, status {:#?}",
        reqwest::StatusCode::BAD_REQUEST
      );
      println!("It's been written a response in the log");
      if let Err(e) = writeln!(
        file,
        "log: error response ticket {} - status: {:#?}",
        &date,
        reqwest::StatusCode::BAD_REQUEST
      ) {
        eprintln!("Couldn't write to file: {}", e);
      }
    }
  }

  Ok(())
}
