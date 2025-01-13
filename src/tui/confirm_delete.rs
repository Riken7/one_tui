use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Clear},
};

pub fn input_folder_name(area: Rect, frame: &mut Frame, input: &String, for_delete: &String) {
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
        .border_type(ratatui::widgets::BorderType::Rounded).border_style(Style::default().fg(Color::Rgb(169, 36, 36)))
        .title(
            Line::from(format!("| To confirm, type '{input}' in box below |"))
                .centered()
                .style(
                    Style::default()
                        .fg(Color::Rgb(252,28,3))
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Rgb(40, 40, 40)));

    frame.render_widget(show_input_box, sub_area);

    let input_line = if for_delete.is_empty() {
        Line::from("".to_string())
    } else {
        Line::from(Span::styled(
            for_delete.clone(),
            Style::default().fg(Color::White),
        ))
    };

    let input_area = Rect {
        x: sub_area.x + 2,
        y: sub_area.y + 2,
        width: width - 4,
        height: 1,
    };
    frame.render_widget(input_line.clone(), input_area);

    let cursor_area = Rect {
        x: sub_area.x + 2 + for_delete.len() as u16, // Position cursor at the end of the input text
        y: sub_area.y + 2,
        width: 1,
        height: 1,
    };
    frame.render_widget(Clear, cursor_area); //cursor rendering
}
