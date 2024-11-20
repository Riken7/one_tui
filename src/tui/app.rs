use std::time::Duration;
use std::thread::sleep;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io::stdout;
use ratatui::crossterm::ExecutableCommand;
use std::time::Instant;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
pub fn main() -> Result<(), std::io::Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    stdout().execute(EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    loop{
        terminal.draw(|frame|{
            frame.render_widget(Block::default().title("Block").borders(Borders::ALL) , frame.area());
        })?;
        if event::poll(Duration::from_millis(200))?{
            if let event::Event::Key(key) = event::read()?{
                if key.kind == KeyEventKind::Press{
                    match key.code{
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

