use crate::api::fetch_folders::Meta;
use chrono::{DateTime, Timelike};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, ListState, Paragraph, Wrap},
};
pub fn render_metadata(area: Rect, frame: &mut Frame, meta_data: &[Meta], state: &mut ListState) {
    let width = std::cmp::max(area.width * 14 / 100 , 22); 
    let height = area.height * 20 / 100;
    let left_margin = 1;
    let top_margin = 1;
    let sub_area = Rect {
        x: area.x + left_margin,
        y: area.y + top_margin + (area.height * 60 / 100),
        width,
        height,
    };
    if meta_data.is_empty() {
        let text = Paragraph::new("No Metadata Available")
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(
                        Line::from("| Metadata |")
                            .centered()
                            .fg(Color::Rgb(232, 117, 26)),
                    )
                    .style(Style::default().fg(Color::Rgb(251, 241, 215))),
            )
            .style(Style::default().fg(Color::Rgb(251, 241, 215)));
        frame.render_widget(text, sub_area);
    }
    if let Some(i) = state.selected() {
        if i >= meta_data.len() {
            return;
        }
        let meta_details = &meta_data[i];

        let size = if meta_details.size > 1024 * 1024 * 1024 {
            format!(
                "{:.2} GB",
                meta_details.size as f64 / (1024.0 * 1024.0 * 1024.0)
            )
        } else if meta_details.size > 1024 * 1024 {
            format!("{:.2} MB", meta_details.size as f64 / (1024.0 * 1024.0))
        } else if meta_details.size > 1024 {
            format!("{:.2} KB", meta_details.size as f64 / 1024.0)
        } else {
            format!("{} B", meta_details.size)
        };

        // \u{1f5c2}
        let last_modified = DateTime::parse_from_rfc3339(&meta_details.last_modified).unwrap();
        let local_last_modified = last_modified.with_timezone(&chrono::Local);
        let date = local_last_modified.format("%d-%m-%Y").to_string();
        let hour = local_last_modified.hour();
        let time = if hour < 12 {
            format!("{:.2}:{:.2} AM", hour, local_last_modified.minute())
        } else {
            format!("{:.2}:{:.2} PM", hour, local_last_modified.minute())
        };

        let detail_text = format!(
            "Size: {}\nChildren: {}\nLastModified:\n{}_{}",
            size, meta_details.children, date, time
        );
        let text = Paragraph::new(detail_text)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(
                        Line::from("| Metadata |")
                            .centered()
                            .fg(Color::Rgb(232, 117, 26)),
                    )
                    .style(Style::default().fg(Color::Rgb(251, 241, 215))),
            )
            .style(Style::default().fg(Color::Rgb(251, 241, 215)));
        frame.render_widget(text, sub_area);
    }
}
