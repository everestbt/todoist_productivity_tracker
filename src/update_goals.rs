use uuid::Uuid; 
use serde::{Deserialize, Serialize};

// Update Daily Goals Request
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub daily_goal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub name: String,
    pub uuid: String,
    pub args: Args,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    commands: Vec<Command>,
}

pub async fn update_daily_goals(key : &String, 
                                daily_goal: &i32) {
    // Make up the json payload
    let payload = Request{
        commands: vec![
            Command{
                name: "update_goals".to_string(),
                uuid: Uuid::new_v4().to_string(),
                args: Args { 
                    daily_goal: *daily_goal 
                }
            }
        ]
    };
    
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .post("https://api.todoist.com/api/v1/sync")
        .header("Authorization", "Bearer ".to_owned() + &key)
        .json(&payload)
        .send()
        .await;
    match req.is_err() {
        true => panic!(),
        false => (),
    }
}