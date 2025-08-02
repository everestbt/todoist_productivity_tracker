use uuid::Uuid; 
use serde::{Deserialize, Serialize};

// Update Daily Goals Request
#[derive(Debug, Serialize, Deserialize)]
struct DailyArgs {
    daily_goal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyCommand {
    #[serde(rename = "type")]
    name: String,
    uuid: String,
    args: DailyArgs,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyRequest {
    commands: Vec<DailyCommand>,
}

pub async fn update_daily_goal(key : &String, 
                                daily_goal: &i32) {
    // Make up the json payload
    let payload = DailyRequest{
        commands: vec![
            DailyCommand{
                name: "update_goals".to_string(),
                uuid: Uuid::new_v4().to_string(),
                args: DailyArgs { 
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

// Update Weekly Goals Request
#[derive(Debug, Serialize, Deserialize)]
struct WeeklyArgs {
    weekly_goal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeeklyCommand {
    #[serde(rename = "type")]
    name: String,
    uuid: String,
    args: WeeklyArgs,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeeklyRequest {
    commands: Vec<WeeklyCommand>,
}

pub async fn update_weekly_goal(key : &String, 
                                weekly_goal: &i32) {
    // Make up the json payload
    let payload = WeeklyRequest{
        commands: vec![
            WeeklyCommand{
                name: "update_goals".to_string(),
                uuid: Uuid::new_v4().to_string(),
                args: WeeklyArgs { 
                    weekly_goal: *weekly_goal 
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