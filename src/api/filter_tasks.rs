use serde::{Deserialize, Serialize};

// Filtered Tasks Request
#[derive(Debug, Serialize, Deserialize)]
pub struct Due {
    pub date: String,
    pub string: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duration {
    pub amount: i32,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub due: Due,
    pub content: String,
    pub priority: i32,
    pub duration: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    results: Vec<Task>,
}

pub async fn get_todays_tasks(key : &str) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=today&limit=200")
        .header("Authorization", "Bearer ".to_owned() + key)
        .send()
        .await;

    if req.is_err() {
        panic!()
    }
    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;

    if response.is_err() {
        panic!("{}",response.unwrap_err().to_string());
    }

    response.unwrap().results
}

pub async fn get_overdue_tasks(key : &str) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=overdue&limit=200")
        .header("Authorization", "Bearer ".to_owned() + key)
        .send()
        .await;
    if req.is_err() {
        panic!();
    }

    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;
    if response.is_err() {
        panic!("{}",response.unwrap_err().to_string());
    }

    response.unwrap().results
}

pub async fn get_tomorrow_tasks(key : &str) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=tomorrow&limit=200")
        .header("Authorization", "Bearer ".to_owned() + key)
        .send()
        .await;

    if req.is_err() {
        panic!()
    }
    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;

    if response.is_err() {
        panic!("{}",response.unwrap_err().to_string());
    }

    response.unwrap().results
}