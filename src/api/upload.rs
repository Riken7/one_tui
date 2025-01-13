use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path:: PathBuf;
pub async fn upload_file(
    access_token: &String,
    file_path: &PathBuf,
    file_size: u64,
    parent_id: &String,
) -> Result<(), Box<dyn Error>> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    
    let mut file = File::open(file_path)?;
    let mut file_buffer = Vec::new();
    if file_size > 30_000_000{
        let client = Client::new();
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/drive/{}/{}:/createUploadSession",
            parent_id,
            file_name
        );
        let response = client
            .post(&url)
            .bearer_auth(access_token)
            .json(&serde_json::json!({
                "item": {
                    "@microsoft.graph.conflictBehavior": "rename"
                }
            }))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(format!("Failed to create upload session: {}", response.status()).into());
        }
        let response_res = response.json::<serde_json::Value>().await?;
        let upload_url = response_res["uploadUrl"].as_str().unwrap();

        let mut buffer = [0; 327_679];
        let mut offset = 0;
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let client = Client::new();
            let response = client
                .put(upload_url)
                .header("Content-Range", format!("bytes {}-{}/{}", offset, offset + bytes_read - 1, file_size))
                .body(buffer[..bytes_read].to_vec())
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(format!("Failed to upload file: {}", response.status()).into());
            }
            offset += bytes_read;
        }


    }
    else {
    file.read_to_end(&mut file_buffer)?;

    let client = Client::new();
    let url = format!(
        "https://graph.microsoft.com/v1.0/me/drive/{}/{}:/content",
        parent_id,
        file_name
    );
    let response = client
        .put(&url)
        .bearer_auth(access_token)
        .body(file_buffer)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to upload file: {}", response.status()).into());
    }
    }
    Ok(())
}
