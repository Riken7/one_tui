use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Padding },
};
use nerd_font_symbols::fa;

use crate::api::fetch_folders::Folder;
pub fn render_root_dir(
    area: Rect,
    frame: &mut Frame,
    folder: &mut [Folder],
    state: &mut ListState,
) {
    let width = std::cmp::max(area.width * 14/100, 22);
    let height = area.height * 60 / 100;
    let left_margin = 1;
    let top_margin = 1;
    let sub_area = Rect {
        x: area.x + left_margin,
        y: area.y + top_margin,
        width,
        height,
    };

    let items: Vec<ListItem> = folder
        .iter_mut()
        .map(|f| ListItem::new(format!("{} {}", f.item_type, f.name)))
        .collect();
//\u{1f3e0} 
    let state_p = if state.selected().is_none() {
        0
    } else {
        state.selected().unwrap() + 1
    };
    let bar = List::new(items)
        .fg(Color::Rgb(251, 241, 215))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(Line::from(format!("| {} one_tui |",fa::FA_FOLDER)).centered().fg(Color::Rgb(253,73,52)))

                //.title(Line::from(format!("| Root dir |")).centered().fg(Color::Rgb(232, 117, 26)))
                .title_bottom(Line::from(format!("| {state_p}/{:?} |",folder.len())).right_aligned()),
        )
        .highlight_style(Style::default().fg(Color::Rgb(142, 192, 124)))
        .highlight_symbol("> ");
    frame.render_stateful_widget(bar, sub_area, state);


    let bg_color = Block::default()
                .borders(Borders::NONE)
                .style(ratatui::style::Style::default().bg(ratatui::style::Color::Rgb(40, 40, 40)));

    frame.render_widget(bg_color, area);

}
