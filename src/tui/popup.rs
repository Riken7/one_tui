use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Clear},
};
pub fn download_location_prompt(
    area: Rect,
    frame: &mut Frame,
    available_paths: &Vec<String>,
    index: &mut ListState,
) -> usize {
    let width = 80;
    let height = 18;

    let sub_area = Rect {
        x: area.x + 40,
        y: area.y + 10,
        width,
        height,
    };
    frame.render_widget(Clear, sub_area);
    let list = available_paths
        .iter()
        .map(|f| ListItem::new(f.to_string()))
        .collect::<Vec<ListItem>>();
    let show_paths = List::new(list).fg(Color::Rgb(251, 241, 215))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(Line::from(format!("| Select download location |")).centered().fg(Color::Rgb(232, 117, 26)))
                .style(Style::default().bg(Color::Rgb(50, 50, 50))),
            )
            .highlight_style(Style::default().fg(Color::Rgb(142, 192, 124)))
            .highlight_symbol("> ");
    frame.render_stateful_widget(show_paths, sub_area, index);
    return index.selected().unwrap_or(0);
}
