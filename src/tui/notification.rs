use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders,List, ListItem },
};
use nerd_font_symbols::fa;
use std::sync::{Arc, Mutex};
pub fn render_notifications(area: Rect, frame: &mut Frame, notification_list : &Arc<Mutex<Vec<String>>>) {
    let mut width = area.width * 34 / 100;
    let mut root_width = area.width * 25 / 100;
    if area.width < 80 {
        width = area.width*33/100;
        root_width = width;
    }
    let top_margin = 0;
    let sub_area = Rect {
        x: area.x + root_width + 1,
        y: area.y + top_margin + (area.height * 80 / 100),
        width,
        height: area.height * 20 / 100,
    };

    let nl = notification_list.lock();
    let notification_items = match nl {
        Ok(ref guard) => guard.iter().rev().map(|f| {
            let msg = format!("{} {}", fa::FA_CIRCLE_EXCLAMATION, f);
            ListItem::new(msg)
        })
        .collect::<Vec<ListItem>>(),
        Err(_) => {
            println!("Failed to lock notification list.");
            Vec::new()
        }
    };

    let list = List::new(notification_items)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(Line::from(format!("| {} Notifications |", fa::FA_ENVELOPE)).centered().fg(Color::Rgb(232, 117, 26)))
                .style(Style::default().fg(Color::Rgb(251, 241, 215))),
        );
    frame.render_widget(list, sub_area);
    
}
