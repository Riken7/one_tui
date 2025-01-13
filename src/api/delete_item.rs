use reqwest::blocking::Client;
use std::error::Error;

pub fn delete_item(
    access_token: &String,
    item_id: &String,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{}",
        item_id
    );
    let _response = client
        .delete(&url)
        .bearer_auth(access_token)
        .send()?;
    Ok(())
}

