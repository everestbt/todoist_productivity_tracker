use std::collections::HashMap;

pub async fn update_task_due(   key : &String, 
                                task_id: &String, 
                                due_date: String, 
                                due_lang: Option<String>,
                                due_string: Option<String>) {
    // Make up the json payload
    let mut map = HashMap::new();
    map.insert("due_date", due_date);
    if due_lang.is_some() {
        map.insert("due_lang", due_lang.unwrap());
    }
    if due_string.is_some() {
        map.insert("due_string", due_string.unwrap());
    }
    
    let req: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .post("https://api.todoist.com/api/v1/tasks/".to_owned() + &task_id)
        .header("Authorization", "Bearer ".to_owned() + &key)
        .json(&map)
        .send()
        .await;
    match req.is_err() {
        true => panic!(),
        false => (),
    }
}