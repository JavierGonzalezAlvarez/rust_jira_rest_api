mod jira;
mod jira_struct;
//mod jira_text;

fn main() {
  env_logger::init();
  dotenv::dotenv().ok();
  if let Err(err) = jira::jira() {
    println!("Error: {:?}", err);
  }
  if let Err(err) = jira_struct::jira() {
    println!("Error connection: {:?}", err);
  }
}
