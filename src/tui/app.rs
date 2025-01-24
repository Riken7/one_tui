use crate::api::{delete_item, download, fetch_folders, mkdir, refetch_folder, upload};
use crate::tui::{
    confirm_delete::input_folder_name,
    download_area::download_block,
    info::{help_popup, render_info},
    input_popup::input_prompt,
    metadata::render_metadata,
    notification::render_notifications,
    popup::download_location_prompt,
    root::render_root_dir,
    sidebar::render_dir,
};
use crossterm::event::{self, KeyCode, KeyModifiers};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use nerd_font_symbols::fa;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::ListState;
use ratatui::Terminal;
use std::{
    env::consts::OS,
    io::{stdout, Write},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::runtime::Runtime;

pub fn run(access_token: String) -> Result<(), std::io::Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;
    stdout().execute(EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut ap = download::load_location();

    let (mut folder, mut meta) = fetch_folders::fetch_folders(&access_token, None).unwrap();

    let mut index = ListState::default();
    index.select(Some(0));
    let mut index2 = ListState::default();
    index2.select(Some(0));
    let mut pop = false;
    let mut input_popup = false;
    let mut confirm_delete = false;
    let mut help = false;
    let native_dialog = true;

    let (mut new_folder, mut new_meta) = if let Some(first_folder) = folder.get(0) {
        fetch_folders::fetch_folders(&access_token, Some(first_folder.id.clone())).unwrap()
    } else {
        (Vec::new(), Vec::new())
    };
    let mut selected_path = String::new();
    let mut flag = 0;

    let mut parent_folder = if let Some(first_folder) = folder.get(0) {
        if first_folder.item_type == "Folder" {
            first_folder.name.clone()
        } else {
            "root".to_string()
        }
    } else {
        "".to_string()
    };
    let mut parent_path = String::from("root:");

    let download_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let message_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let rt = Runtime::new()?;

    let mut folder_name = String::new();
    let mut entered_folder = String::new();
    //let term = std::env::var("TERM").unwrap();

    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            if flag == 0 {
                render_metadata(size, frame, &meta, &mut index);
                render_root_dir(size, frame, &mut folder, &mut index);
                render_dir(
                    size,
                    frame,
                    &mut new_folder,
                    &mut ListState::default(),
                    parent_folder.clone(),
                );
                if pop {
                    download_location_prompt(size, frame, &mut ap, &mut index2);
                }
            }
            if flag == 1 {
                render_metadata(size, frame, &new_meta, &mut index);
                render_root_dir(size, frame, &mut folder, &mut ListState::default());
                render_dir(
                    size,
                    frame,
                    &mut new_folder,
                    &mut index,
                    parent_folder.clone(),
                );
                if pop {
                    download_location_prompt(size, frame, &mut ap, &mut index2);
                }
            }
            download_block(size, frame, &download_list);
            render_info(size, frame);
            render_notifications(size, frame, &message_list);
            if input_popup {
                input_prompt(size, frame, &folder_name);
            }
            if confirm_delete {
                input_folder_name(size, frame, &folder_name, &entered_folder);
            }
            if help {
                help_popup(size, frame);
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                if input_popup || confirm_delete {
                    match key.code {
                        KeyCode::Esc => {
                            input_popup = false;
                            confirm_delete = false;
                            folder_name.clear();
                            entered_folder.clear();
                        }
                        KeyCode::Char(c) => {
                            if confirm_delete {
                                entered_folder.push(c);
                            } else {
                                folder_name.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            if confirm_delete {
                                entered_folder.pop();
                            } else {
                                folder_name.pop();
                            }
                        }
                        KeyCode::Enter => {
                            if confirm_delete && !folder_name.is_empty() {
                                let (folders, metas) = if flag == 0 {
                                    (&mut folder, &mut meta)
                                } else {
                                    (&mut new_folder, &mut new_meta)
                                };
                                let item =
                                    folders.get(index.selected().unwrap()).unwrap().id.clone();
                                if entered_folder == folder_name {
                                    let _ = delete_item::delete_item(&access_token, &item);
                                    message_list
                                        .lock()
                                        .unwrap()
                                        .push("Item deleted".to_string());
                                    folders.remove(index.selected().unwrap());
                                    metas.remove(index.selected().unwrap());
                                } else {
                                    message_list
                                        .lock()
                                        .unwrap()
                                        .push("Folder name does not match".to_string());
                                }
                                confirm_delete = false;
                                entered_folder.clear();
                            }
                            if input_popup && !folder_name.is_empty() {
                                folder_name = folder_name.trim().to_string();
                                let parent_folder_id = if flag == 0 {
                                    "root".to_string()
                                } else {
                                    if let Some(ff) = new_folder.get(0) {
                                        ff.parent_folder_id.clone()
                                    } else {
                                        format!("{parent_path}:")
                                    }
                                };
                                let _ = mkdir::create_folder(
                                    &access_token,
                                    &parent_folder_id,
                                    &folder_name,
                                    &message_list,
                                );
                                if flag == 0 {
                                    (folder, meta) =
                                        refetch_folder::refresh_list(&access_token, None);
                                }
                                if flag == 1 {
                                    (new_folder, new_meta) = refetch_folder::refresh_list(
                                        &access_token,
                                        Some(format!("{parent_path}:")),
                                    );
                                }
                                index.select(Some(0));
                                input_popup = false;
                            }
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => {
                            if pop{
                                pop = false
                            }
                        }
                        KeyCode::Char('c') => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                break;
                            }
                        }
                        KeyCode::Char('j') => {
                            if !pop {
                                if index.selected().is_none() {
                                    if (flag == 0 && !folder.is_empty())
                                        || (flag == 1 && !new_folder.is_empty())
                                    {
                                        index.select(Some(0));
                                    }
                                }
                                if let Some(i) = index.selected() {
                                    if (flag == 0 && i < folder.len() - 1)
                                        || (flag == 1 && i < new_folder.len() - 1)
                                    {
                                        index.select_next();
                                    }
                                }
                            }
                            if pop {
                                if let Some(i) = index2.selected() {
                                    if i < ap.len() - 1 {
                                        index2.select_next();
                                    }
                                }
                            }
                        }
                        KeyCode::Char('k') => {
                            if !pop {
                                if let Some(i) = index.selected() {
                                    if i > 0 {
                                        index.select_previous();
                                    }
                                }
                            }
                            if pop {
                                if let Some(i) = index2.selected() {
                                    if i > 0 {
                                        index2.select_previous();
                                    }
                                }
                            }
                        }
                        KeyCode::Char('a') => {
                            input_popup = true;
                            folder_name.clear();
                        }
                        KeyCode::Char('r') => {
                            if (OS == "macos" && key.modifiers == KeyModifiers::ALT)
                                || (OS == "linux" && key.modifiers == KeyModifiers::CONTROL)
                            {
                                (folder, meta) = refetch_folder::refresh_list(&access_token, None);
                                (new_folder, new_meta) = refetch_folder::refresh_list(
                                    &access_token,
                                    Some(format!("{parent_path}:")),
                                );
                                message_list
                                    .lock()
                                    .unwrap()
                                    .push("session refreshed".to_string());
                            }
                        }
                        KeyCode::Char('h') => {
                            if (OS == "macos" && key.modifiers == KeyModifiers::ALT)
                                || (OS == "linux" && key.modifiers == KeyModifiers::CONTROL)
                            {
                                help = !help;
                            }
                        }
                        KeyCode::Char('x') => {
                            if (OS == "macos" && key.modifiers == KeyModifiers::ALT)
                                || (OS == "linux" && key.modifiers == KeyModifiers::CONTROL)
                            {
                                let selected_list = if flag == 0 { &folder } else { &new_folder };
                                match index.selected() {
                                    Some(i) => {
                                        let item_name = &selected_list.get(i).unwrap().name;
                                        folder_name = item_name.clone();

                                        confirm_delete = true;
                                    }
                                    None => {
                                        message_list
                                            .lock()
                                            .unwrap()
                                            .push("No item to delete".to_string());
                                        continue;
                                    }
                                };
                            }
                        }
                        KeyCode::Enter => {
                            if !pop {
                                if let Some(id) = index.selected() {
                                    if flag == 0 && meta.get(id).unwrap().item_type == "Folder" {
                                        let current_folder = folder.get(id).unwrap().id.clone();
                                        parent_folder = folder.get(id).unwrap().name.clone();
                                        parent_path = format!("{}/{}", parent_path, parent_folder);
                                        let (new_folder_list, new_meta_list) =
                                            fetch_folders::fetch_folders(
                                                &access_token,
                                                Some(current_folder),
                                            )
                                            .unwrap();
                                        new_folder = new_folder_list;
                                        new_meta = new_meta_list;
                                        flag = 1;
                                        index = ListState::default();
                                        index.select(Some(0));
                                    } else if flag == 1
                                        && new_meta.get(id).unwrap().item_type == "Folder"
                                    {
                                        let current_folder = new_folder.get(id).unwrap().id.clone();
                                        parent_folder = new_folder.get(id).unwrap().name.clone();
                                        parent_path = format!("{}/{}", parent_path, parent_folder);
                                        let (new_folder_list, new_meta_list) =
                                            fetch_folders::fetch_folders(
                                                &access_token,
                                                Some(current_folder),
                                            )
                                            .unwrap();
                                        new_folder = new_folder_list;
                                        new_meta = new_meta_list;
                                        index = ListState::default();
                                        index.select(Some(0));
                                    }
                                }
                            }
                            if pop {
                                if let Some(i) = index2.selected() {
                                    if i < ap.len() {
                                        selected_path = ap[i].clone();
                                        pop = !pop;
                                    }
                                }
                                if let Some(id) = index.selected() {
                                    if (flag == 1 && new_meta.get(id).unwrap().item_type == "File")
                                        || (flag == 0 && meta.get(id).unwrap().item_type == "File")
                                    {
                                        let id = index.selected().unwrap();
                                        let access_token = access_token.clone();
                                        let folder_id;
                                        let file_name;
                                        if flag != 0 {
                                            folder_id = new_folder[id].clone().id;
                                            file_name = new_folder[id].clone().name;
                                        } else {
                                            folder_id = folder[id].clone().id;
                                            file_name = folder[id].clone().name;
                                        }
                                        let dest = selected_path.clone();

                                        let new_dl = Arc::clone(&download_list);
                                        {
                                            let mut new_dl_lock = new_dl.lock().unwrap();
                                            new_dl_lock.push(format!(
                                                "{} {} | Downloading...",
                                                fa::FA_CIRCLE_DOWN,
                                                file_name.clone()
                                            ));
                                        }
                                        rt.block_on(async {
                                            tokio::spawn(async move {
                                                match download::download_file(
                                                    &access_token,
                                                    &folder_id,
                                                    &file_name,
                                                    &dest,
                                                )
                                                .await
                                                {
                                                    Ok(_) => {
                                                        let mut new_dl_lock =
                                                            new_dl.lock().unwrap();
                                                        if let Some(i) =
                                                            new_dl_lock.iter().position(|x| {
                                                                *x == format!(
                                                                    "{} {} | Downloading...",
                                                                    fa::FA_CIRCLE_DOWN,
                                                                    &file_name
                                                                )
                                                            })
                                                        {
                                                            new_dl_lock[i] = format!(
                                                                "{} {} | Downloaded successfully",
                                                                fa::FA_CIRCLE_CHECK,
                                                                file_name.clone()
                                                            );
                                                        }
                                                    }
                                                    Err(_e) => {
                                                        let mut new_dl_lock =
                                                            new_dl.lock().unwrap();
                                                        if let Some(i) =
                                                            new_dl_lock.iter().position(|x| {
                                                                *x == format!(
                                                                    "{} {} | Downloading...",
                                                                    fa::FA_CIRCLE_DOWN,
                                                                    &file_name
                                                                )
                                                            })
                                                        {
                                                            new_dl_lock[i] = format!(
                                                                "{} {} | Download Failed",
                                                                fa::FA_CIRCLE_EXCLAMATION,
                                                                file_name.clone()
                                                            );
                                                        }
                                                    }
                                                }
                                            });
                                        });
                                    } else {
                                        message_list
                                            .lock()
                                            .unwrap()
                                            .push("Select a file to download".to_string());
                                    }
                                    index.select(index.selected());
                                } else {
                                    message_list
                                        .lock()
                                        .unwrap()
                                        .push("Select a file to download".to_string());
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            flag = 0;
                            index.select(Some(0));
                            parent_folder = "root".to_string();
                            parent_path = "root:".to_string();
                        }
                        KeyCode::Char('d') => {
                            if (OS == "macos" && key.modifiers == KeyModifiers::ALT)
                                || (OS == "linux" && key.modifiers == KeyModifiers::CONTROL)
                            {
                                pop = !pop;
                            }
                        }
                        KeyCode::Char('u') => {
                            if (OS == "macos" && key.modifiers == KeyModifiers::ALT)
                                || (OS == "linux" && key.modifiers == KeyModifiers::CONTROL)
                            {
                                let access_token_ = access_token.clone();
                                let parent_id = parent_path.clone();
                                if native_dialog {
                                    let file_path = FileDialog::new()
                                        .set_location("~/")
                                        .add_filter("All Files", &["*"])
                                        .show_open_single_file();

                                    let file_path = match file_path {
                                        Ok(Some(file_path)) => file_path,
                                        Ok(None) => {
                                            message_list
                                                .lock()
                                                .unwrap()
                                                .push("No file selected".to_string());
                                            continue;
                                        }
                                        Err(_e) => {
                                            message_list.lock().unwrap().push(format!(
                                                "download zenity/kdialog for file dialog"
                                            ));
                                            continue;
                                        }
                                    };
                                    let file_name = file_path
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string();
                                    let file_size = file_path.metadata().unwrap().len();
                                    let dialog_message = MessageDialog::new()
                                        .set_type(MessageType::Info)
                                        .set_title("confirm upload ? ")
                                        .set_text(&format!("{:?}", file_path))
                                        .show_confirm()
                                        .unwrap();

                                    if dialog_message {
                                        let new_dl = Arc::clone(&download_list);
                                        {
                                            let mut new_dl_lock = new_dl.lock().unwrap();
                                            new_dl_lock.push(format!(
                                                "{} {} | Uploading...",
                                                fa::FA_CIRCLE_UP,
                                                file_name.clone()
                                            ));
                                        }
                                        rt.block_on(async {
                                            tokio::spawn(async move {
                                                match upload::upload_file(
                                                    &access_token_,
                                                    &file_path,
                                                    file_size,
                                                    &parent_id,
                                                )
                                                .await
                                                {
                                                    Ok(_) => {
                                                        let mut new_dl_lock =
                                                            new_dl.lock().unwrap();
                                                        if let Some(i) =
                                                            new_dl_lock.iter().position(|x| {
                                                                *x == format!(
                                                                    "{} {} | Uploading...",
                                                                    fa::FA_CIRCLE_UP,
                                                                    &file_name
                                                                )
                                                            })
                                                        {
                                                            new_dl_lock[i] = format!(
                                                                "{} {} | Uploaded successfully",
                                                                fa::FA_CIRCLE_CHECK,
                                                                file_name.clone()
                                                            );
                                                        }
                                                    }
                                                    Err(_e) => {
                                                        let mut new_dl_lock =
                                                            new_dl.lock().unwrap();
                                                        if let Some(i) =
                                                            new_dl_lock.iter().position(|x| {
                                                                *x == format!(
                                                                    "{} {} | Uploading...",
                                                                    fa::FA_CIRCLE_UP,
                                                                    &file_name
                                                                )
                                                            })
                                                        {
                                                            new_dl_lock[i] = format!(
                                                                "{} {} | Upload Failed",
                                                                fa::FA_CIRCLE_EXCLAMATION,
                                                                file_name.clone()
                                                            );
                                                        }
                                                    }
                                                }
                                            });
                                        });
                                    }
                                }
                                terminal.clear()?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().flush()?;

    Ok(())
}
