use crate::api::fetch_folders::Folder;
use nerd_font_symbols::{cod, fa};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
pub fn render_dir(
    area: Rect,
    frame: &mut Frame,
    folder: &mut [Folder],
    state: &mut ListState,
    parent_folder: String,
) {
    let mut width = std::cmp::max(area.width * 45 / 100, 72);
    let root_width = std::cmp::max(area.width * 14 / 100, 22);
    let height = area.height * 78 / 100;
    let top_margin = 1;
    if area.width < 80 {
        width = area.width - root_width;
    }
    let sub_area = Rect {
        x: area.x + root_width + 1,
        y: area.y + top_margin,
        width,
        height,
    };
    let items: Vec<ListItem> = folder
        .iter_mut()
        .map(|f| ListItem::new(format!("{} {}", f.item_type, f.name)))
        .collect();

    let state_p = if state.selected().is_none() {
        0
    } else {
        state.selected().unwrap() + 1
    };

    let parent_folder = parent_folder.clone();
    let bar = List::new(items)
        .fg(Color::Rgb(251, 241, 215))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(
                    Line::from(format!("| {} /../{}/ |", fa::FA_FOLDER, parent_folder))
                        .centered()
                        .fg(Color::Rgb(232, 117, 26)),
                )
                //.title(Line::from(format!("| \u{f07b} one_tui |")).centered().fg(Color::Rgb(253,73,52))),
                //.title(Line::from(format!("| \u{f07b} one_tui |")).centered().fg(Color::Rgb(204,36,29))),
                .title_bottom(
                    Line::from(format!("| {}/{:?} |", state_p, folder.len())).right_aligned(),
                ),
        )
        .highlight_style(Style::default().fg(Color::Rgb(142, 192, 124)))
        .highlight_symbol("> ");
    frame.render_stateful_widget(bar, sub_area, state);
    render_preview2(area, frame, state);
}
pub fn render_preview2(area: Rect, frame: &mut Frame<'_>, _index: &mut ListState) {
    let width = area.width * 40 / 100;
    let height = area.height * 78 / 100;
    let root_width = area.width * 59 / 100;
    let top_margin = 1;
    if area.width < 150 {
        return;
    }
    let sub_area = Rect {
        x: area.x + root_width + 1,
        y: area.y + top_margin,
        width,
        height,
    };
    let bar = Paragraph::new("Preview Unavailable right now")
        .centered()
        .fg(Color::Rgb(250, 241, 215))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .padding(Padding::new(0, 1, 1, 1))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(
                    Line::from(format!("| {} Preview Block |", cod::COD_PREVIEW))
                        .centered()
                        .fg(Color::Rgb(232, 117, 26)),
                ),
        );

    frame.render_widget(bar, sub_area);
}
pub fn render_preview(area: Rect, frame: &mut Frame, folder: &mut [Folder]) {
    let width = area.width * 40 / 100;
    let height = area.height * 78 / 100;
    let root_width = area.width * 59 / 100;
    let top_margin = 1;
    if area.width < 150 {
        return;
    }
    let sub_area = Rect {
        x: area.x + root_width + 1,
        y: area.y + top_margin,
        width,
        height,
    };
    if folder.len() == 0 {
        let text = Paragraph::new("No Preview Available")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 1, 1, 1))
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(
                        Line::from(format!("| {} Preview Block |", cod::COD_PREVIEW))
                            .centered()
                            .fg(Color::Rgb(232, 117, 26)),
                    ),
            )
            .style(Style::default().fg(Color::Rgb(251, 241, 215)));
        frame.render_widget(text, sub_area);
    }
    let items: Vec<ListItem> = folder
        .iter_mut()
        .map(|f| ListItem::new(format!("{} {}", f.item_type, f.name)))
        .collect();
    let list = List::new(items).block(
        Block::new()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(
                Line::from(format!("| {} Preview Block |", cod::COD_PREVIEW))
                    .centered()
                    .fg(Color::Rgb(232, 117, 26)),
            )
            .title_bottom(Line::from(format!("| {}/{:?} |", 1, folder.len())).right_aligned()),
    );

    frame.render_widget(list, sub_area);
}
