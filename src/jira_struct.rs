//Creating JSON by serializing data structures
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Type3 {
  #[serde(rename = "type")]
  type4: String,
  text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Type2 {
  #[serde(rename = "type")]
  type2: String,
  content: [Type3; 1],
}

#[derive(Debug, Serialize, Deserialize)]
struct Description {
  #[serde(rename = "type")]
  type1: String,
  version: i32,
  content: [Type2; 1],
}

/*
"description": {
  "type": "doc",
  "version": 1,
  "content": [
    {
      "type": "paragraph",
      "content": [
        {
          "text": "Data have changed jira text",
          "type": "text"
        }
      ]
    }
  ]
},
*/

#[derive(Debug, Serialize, Deserialize)]
struct Customfield10029 {
  id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
  key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Issuetype {
  id: String,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fields {
  summary: String,
  issuetype: Issuetype,
  project: Project,
  customfield_10029: Customfield10029,
  description: Description,
}

#[derive(Debug, Serialize, Deserialize)]
struct Obj {
  fields: Fields,
}

#[tokio::main]
pub async fn jira() -> Result<(), reqwest::Error> {
  let summary: String = String::from("Jira: Issue 1");
  let data = Obj {
    fields: Fields {
      summary: summary.into(),
      project: Project { key: "DJAN".into() },
      issuetype: Issuetype {
        id: "10001".into(),
        name: "Story".into(),
      },
      customfield_10029: Customfield10029 { id: "10021".into() },
      description: Description {
        type1: "doc".into(),
        version: 1,
        content: [Type2 {
          type2: "paragraph".into(),
          content: [Type3 {
            text: "Data have changed jira struct".into(),
            type4: "text".into(),
          }],
        }],
      },
      //
    },
  };

  /*
  "description": {
    "type": "doc",
    "version": 1,
    "content": [
      {
        "type": "paragraph",
        "content": [
          {
            "text": "Data have changed jira text",
            "type": "text"
          }
        ]
      }
    ]
  },
  */

  println!("body no serialized(json) => {:#?}", &data);
  let serialized = serde_json::to_string_pretty(&data).unwrap();
  println!("body serialized and \n (json) => {:#?}", &serialized);
  println!("body serialized (json) => {}", &serialized);

  let userjira = std::env::var("USERJIRA").expect("user");
  let token = std::env::var("TOKEN").expect("token");

  let resp = reqwest::Client::new()
    .post("https://jira1414.atlassian.net/rest/api/3/issue")
    .header("Content-Type", "application/json")
    .basic_auth(userjira, Some(token))
    .body(serialized)
    .send()
    .await?
    .text()
    .await?;
  println!("{:#?}", resp);
  let valor: Value = serde_json::from_str::<Value>(&resp).unwrap();
  println!("{:#?}", valor);
  println!("{:#?}", valor.get("errorMessages"));
  let error = valor.get("errorMessages");
  match error {
    Some(_) => {
      println!("found an error in the response");
    }
    _ => println!("no errors"),
  }

  Ok(())
}
