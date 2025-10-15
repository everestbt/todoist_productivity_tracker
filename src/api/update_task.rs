use std::collections::HashMap;

pub async fn update_task_due(   key : &str, 
                                task_id: &str, 
                                due_date: String, 
                                due_lang: Option<String>,
                                due_string: Option<String>) {
    // Make up the json payload
    let mut map = HashMap::new();
    map.insert("due_date", due_date);
    if let Some(x) = due_lang {
        map.insert("due_lang", x);
    }
    if let Some(x) = due_string {
        map.insert("due_string", x);
    }
    
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .post("https://api.todoist.com/api/v1/tasks/".to_owned() + task_id)
        .header("Authorization", "Bearer ".to_owned() + key)
        .json(&map)
        .send()
        .await;
    if req.is_err() {
        panic!()
    }
}