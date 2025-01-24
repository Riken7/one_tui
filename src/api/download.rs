use reqwest::Client;
use std::fs::{File,create_dir_all};
use std::io::Write;
use std::path::Path;
use std::error::Error;

use toml;
use serde::{Deserialize, Serialize};
use std::fs;
use dirs_next;

#[derive(Deserialize,Serialize)]
struct Config {
    download_path: DownloadPaths,
}
#[derive(Deserialize,Serialize)]
struct DownloadPaths {
    paths: Vec<String>,
}
pub fn load_location() -> Vec<String> {
    let config_path = get_config_path();
    if !config_path.exists(){
        if let Err(e) = create_default_config(&config_path){
            eprintln!("Failed to create default config: {}", e);
            std::process::exit(1);
        }
    }
    let config_data = fs::read_to_string(config_path).expect("Unable to read file");
    let config: Config = toml::de::from_str(&config_data).expect("Unable to parse file");

    config.download_path.paths
}
fn get_config_path() -> std::path::PathBuf {
    let home_dir = dirs_next::home_dir().expect("Unable to get home directory");
    let config_dir = home_dir.join(".config").join("onetui");
    create_dir_all(&config_dir).expect("Unable to create config directory");
    config_dir.join("config.toml")
}
fn create_default_config(config_path : &Path) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs_next::home_dir().expect("Unable to get home directory");
    let default_download_dir = home_dir.join("Downloads");
    let default_config = Config{
        download_path: DownloadPaths{
            paths: vec![default_download_dir.to_str().unwrap().to_string()],
        },
    };
    let toml_string = toml::to_string(&default_config)?;

    let mut file = File::create(config_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

pub async fn download_file(
    access_token: &String,
    item_id: &String,
    name : &String,
    download_path : &String,
) -> Result<(), Box<dyn Error>> {
    if !Path::new(&download_path).exists(){
        create_dir_all(&download_path)?;
        //println!("Created directory: {}", download_path);
    }
    let client = Client::new();
    let url = format!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{}/content",
        item_id
    );
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }
    let download_file_path = format!("{}/{}", download_path,name);
    save_file(&download_file_path, response).await?;
    Ok(())
}
async fn save_file(download_file_path: &String, response: reqwest::Response) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&download_file_path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}
