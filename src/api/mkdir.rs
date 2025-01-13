use reqwest::blocking::Client;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub fn create_folder(
    access_token: &String,
    parent_folder_id: &String,
    new_folder_name: &String,
    message_list : &Arc<Mutex<Vec<String>>>
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{}/children",
        parent_folder_id
    );
    let body = format!(
        r#"{{"name":"{}","folder":{{ }},"@microsoft.graph.conflictBehavior":"rename"}}"#,
        new_folder_name
    );
    let response = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    if response.status().is_success() {
        message_list.lock().unwrap().push(format!("folder created successfully"));
    } else {
        message_list.lock().unwrap().push(format!("Error creating folder: {}", response.status()));
    }

    Ok(())
}

