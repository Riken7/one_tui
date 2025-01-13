use nerd_font_symbols::fa;
use reqwest::blocking::Client;
use std::error::Error;
#[derive(Debug, Clone, PartialEq)]
pub enum ItemTypes {
    Folder,
    File,
}
#[derive(Debug, Clone)]
pub struct Meta {
    pub size: i64,
    pub last_modified: String,
    pub children: i64,
    pub item_type: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Folder {
    pub name: String,
    pub id: String,
    pub item_type: String,
    pub parent_folder_id: String,
}

impl IntoIterator for Folder {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let fields = vec![self.name, self.id];
        fields.into_iter()
    }
}

pub fn fetch_folders(
    access_token: &String,
    folder_id: Option<String>,
) -> Result<(Vec<Folder>, Vec<Meta>), Box<dyn Error>> {
    let client = Client::new();
    let url = if let Some(ref id) = folder_id {
        format!(
            "https://graph.microsoft.com/v1.0/me/drive/items/{}/children",
            id
        )
    } else {
        "https://graph.microsoft.com/v1.0/me/drive/root/children".to_string()
    };
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()?
        .json::<serde_json::Value>()?;

    let mut meta_data = Vec::new();
    let mut folders = Vec::new();
    for item in response["value"].as_array().unwrap_or(&vec![]) {
        let name = item["name"].as_str().unwrap_or("Unknown").to_string();
        let id = item["id"].as_str().unwrap_or("").to_string();
        let item_type = if item.get("folder").is_some() {
            ItemTypes::Folder
        } else {
            ItemTypes::File
        };
        let parent_folder_id = item["parentReference"]["id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        //let combined_name = match item_type {
        //    ItemTypes::Folder => format!("\u{f07b} {}", name),
        //    ItemTypes::File => format!("\u{f15b} {}", name),
        //};
        let folder = Folder {
            name: name.clone(),
            id: id.clone(),
            item_type: if item_type == ItemTypes::Folder {
                format!("{}", fa::FA_FOLDER)
            } else {
                format!("{}", fa::FA_FILE)
            },
            parent_folder_id: parent_folder_id.clone(),
        };
        let meta = Meta {
            size: item["size"].as_i64().unwrap_or(0),
            last_modified: item["lastModifiedDateTime"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            children: item["folder"]["childCount"].as_i64().unwrap_or(0),
            item_type: if item.get("folder").is_some() {
                "Folder".to_string()
            } else {
                "File".to_string()
            },
        };
        meta_data.push(meta);
        folders.push(folder);
    }
    Ok((folders, meta_data))
}
