use reqwest::blocking::{Client, Response};
use reqwest::header::{AUTHORIZATION , CONTENT_TYPE};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct FileItem{
    name : String,
}
#[derive(Deserialize, Debug)]
struct FileListResponse{
    value : Vec<FileItem>,
}

pub fn list_files(access_token: Result<String , Box<dyn Error>>) -> Result<(), Box<dyn Error>>{
    let access_token = access_token?;
    println!("Access Token: {}", access_token);
    let url = "https://graph.microsoft.com/v1.0/me/drive/root/children";
    let client = Client::new();
    let response: Response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .header(CONTENT_TYPE, "application/json")
        .send()?;
        if response.status().is_success() {
        let file_list: FileListResponse  = response.json()?;
        
        // Print the list of files and folders
        if file_list.value.is_empty() {
            println!("No files or folders found in the root directory.");
        } else {
            for file in file_list.value {
                println!("Name: {}" , file.name);
            }
        }
    } else {
        eprintln!("Error fetching files: {}", response.status());
    }

    Ok(())
}
