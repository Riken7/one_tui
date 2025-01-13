use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders , List, ListItem},
};
use nerd_font_symbols::fa;
use std::sync::{Arc, Mutex};
pub fn download_block(area: Rect, frame: &mut Frame, download_list: &Arc<Mutex<Vec<String>>>) {
    let mut width = area.width * 40 / 100;
    let mut root_width = area.width * 59 / 100;
    if area.width < 80 {
        width = area.width * 33 / 100;
        root_width = area.width * 66 / 100;
    }
    let height = area.height * 20 / 100;
    let _left_margin = 0;
    let top_margin = 0;
    let sub_area = Rect {
        x: area.x + root_width +1 ,
        y: area.y + top_margin + (area.height * 80 / 100),
        width,
        height,
    };
    let dl = download_list.lock();
        let download_items = match dl {
        Ok(ref guard) => guard.iter().rev().map(|f| ListItem::new(f.to_string())).collect::<Vec<ListItem>>(),
        Err(_) => {
            println!("Failed to lock download list.");
            Vec::new()
        }
    };
     
    let list = List::new(download_items)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(Line::from(format!("| {} Transfer Status |", fa::FA_CLOUD_ARROW_UP)).centered().fg(Color::Rgb(232, 117, 26)))
                .style(Style::default().fg(Color::Rgb(251, 241, 215))),
        )
        .highlight_style(Style::default().fg(Color::Rgb(142, 192, 124)))
        .highlight_symbol("> ");
    frame.render_widget(list, sub_area);
}
