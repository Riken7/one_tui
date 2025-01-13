use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    text::{Span, Line},
    layout::Rect,
    widgets::{Block, Borders, Clear},
};

pub fn input_prompt(
    area: Rect,
    frame: &mut Frame,
    input: &String
) {
    let width = 50;
    let height = 5;

    let sub_area = Rect {
        x: area.x + 40,
        y: area.y + 15,
        width,
        height,
    };

    frame.render_widget(Clear, sub_area);

    let show_input_box = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Line::from("| Enter Folder Name |").centered().style(Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)))
        .style(Style::default().bg(Color::Rgb(40, 40, 40)));

    frame.render_widget(show_input_box, sub_area);

    let input_line = if input.is_empty() {
        Line::from("".to_string())
    } else {
        Line::from(Span::styled(input.clone(), Style::default().fg(Color::White)))
    };

    let input_area = Rect {
        x: sub_area.x + 2,
        y: sub_area.y + 2,
        width: width - 4,
        height: 1,
    };
    frame.render_widget(input_line, input_area);

    let cursor_area = Rect {
        x: sub_area.x + 2 + input.len() as u16,
        y: sub_area.y + 2,
        width: 1,
        height: 1,
    };
    frame.render_widget(Clear, cursor_area); //cursor rendering
}

