use crate::api::fetch_folders;
pub fn refresh_list(
    access_token: &String,
    folder_id: Option<String>,
) -> (Vec<fetch_folders::Folder>, Vec<fetch_folders::Meta>) {
    if folder_id.is_none() {
        fetch_folders::fetch_folders(&access_token, None).unwrap()
    } else {
        fetch_folders::fetch_folders(&access_token, folder_id).unwrap()
    }
}
