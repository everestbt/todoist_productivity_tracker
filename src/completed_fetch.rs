use serde::{Deserialize, Serialize};

// Completed Stats Request
#[derive(Debug, Serialize, Deserialize)]
pub struct Goals {
    pub daily_goal: i32,
    pub weekly_goal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayItem {
    pub date: String,
    pub total_completed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekItem {
    pub from: String,
    pub total_completed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedStats {
    pub days_items: Vec<DayItem>,
    pub week_items: Vec<WeekItem>,
    pub goals: Goals,
}

pub async fn get_completed_stats(key : &String) -> CompletedStats {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/completed/stats")
        .header("Authorization", "Bearer ".to_owned() + &key)
        .send()
        .await;

    match req.is_err() {
        true => panic!(),
        false => (),
    }
    let response: Result<CompletedStats, reqwest::Error> = req.unwrap()
        .json()
        .await;

    match response.is_err() {
        true => panic!(),
        false => (),
    }

    response.unwrap()
}