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

    if let Err(r) = req {
        log::error!("Failed to send the request for today tasks: {}", r);
        panic!("Failed to send the request for today tasks")
    }
    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;

    if let Err(r) = response {
        log::error!("Failed to get a response for today tasks: {}", r);
        panic!("Failed to get a response for today tasks");
    }

    response.unwrap().results
}

pub async fn get_overdue_tasks(key : &str) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=overdue&limit=200")
        .header("Authorization", "Bearer ".to_owned() + key)
        .send()
        .await;
    if let Err(r) = req {
        log::error!("Failed to send the request for overdue tasks: {}", r);
        panic!("Failed to send the request for overdue tasks")
    }

    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;
    if let Err(r) = response {
        log::error!("Failed to get a response for overdue tasks: {}", r);
        panic!("Failed to get a response for overdue tasks");
    }

    response.unwrap().results
}

pub async fn get_tomorrow_tasks(key : &str) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=tomorrow&limit=200")
        .header("Authorization", "Bearer ".to_owned() + key)
        .send()
        .await;

    if let Err(r) = req {
        log::error!("Failed to send the request for tomorrow tasks: {}", r);
        panic!("Failed to send the request for tomorrow tasks")
    }
    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;

    if let Err(r) = response {
        log::error!("Failed to get a response for tomorrow tasks: {}", r);
        panic!("Failed to get a response for tomorrow tasks");
    }

    response.unwrap().results
}