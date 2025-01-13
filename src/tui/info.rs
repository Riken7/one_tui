use nerd_font_symbols::fa;
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};
pub fn help_popup(area: Rect, frame: &mut Frame) {
    let width = 80;
    let height = 18;

    let sub_area = Rect {
        x: area.x + 40,
        y: area.y + 10,
        width,
        height,
    };
    let keybindings = "\
    - [j/k] Down/Up\n\
    - [ctrl+c] Quit\n\
    - [ctrl+d] Download file\n\
    - [ctrl+u] Upload file\n\
    - [ctrl+x] Delete an item\n\
    - [ctrl+r] Refresh the current folder\n\
    - [a] Create a new folder\n\
    - [Backspace] Toggle to root folder\n\
    - [Ctrl+h] Show help popup\n\n\
    - To set a custom download location, you can edit the `config.toml` file inside the `.config/onetui/` directory.\n";

    let help_text = Paragraph::new(keybindings)
        .wrap(Wrap { trim: true })
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(
                    Line::from(format!("| {} Help |", fa::FA_CIRCLE_INFO))
                        .centered()
                        .fg(Color::Rgb(232, 117, 26)),
                )
                .style(Style::default().bg(Color::Rgb(50, 50, 50))),
        );
    frame.render_widget(help_text, sub_area);
}
pub fn render_info(area: Rect, frame: &mut Frame) {
    let mut width = area.width * 25 / 100;
    if area.width < 80 {
        width = area.width * 33 / 100;
    }
    let height = area.height * 20 / 100;
    let left_margin = 1;
    let top_margin = 0;
    let sub_area = Rect {
        x: area.x + left_margin,
        y: area.y + top_margin + (area.height * 80 / 100),
        width,
        height,
    };
    let help = "-[ctrl+c] Quit\n-[j/k] Down/Up\n-[ctrl+h] Show help Popup\n-[ctrl+u] Upload file\n-[Backspace] Toggle to root folder".to_string();
    let help_text = Paragraph::new(help).wrap(Wrap { trim: true }).block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(
                Line::from(format!("| {} Help |", fa::FA_CIRCLE_INFO))
                    .centered()
                    .fg(Color::Rgb(232, 117, 26)),
            )
            .style(Style::default().fg(Color::Rgb(251, 241, 215))),
    );
    frame.render_widget(help_text, sub_area);
}
