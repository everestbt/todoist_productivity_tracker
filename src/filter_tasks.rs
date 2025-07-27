use serde::{Deserialize, Serialize};

// Filtered Tasks Request
#[derive(Debug, Serialize, Deserialize)]
pub struct Due {
    pub date: String,
    pub string: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub due: Due,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    results: Vec<Task>,
}

pub async fn get_todays_tasks(key : &String) -> Vec<Task> {
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get("https://api.todoist.com/api/v1/tasks/filter?query=today&limit=200")
        .header("Authorization", "Bearer ".to_owned() + &key)
        .send()
        .await;

    match req.is_err() {
        true => panic!(),
        false => (),
    }
    let response: Result<Response, reqwest::Error> = req.unwrap()
        .json()
        .await;

    match response.is_err() {
        true => panic!("{}",response.unwrap_err().to_string()),
        false => (),
    }

    response.unwrap().results
}