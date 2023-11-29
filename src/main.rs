use std::io::{stdout, Result};

use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let mut area = frame.size();
            area.height = area.height / 2;
            area.width = area.width / 2;
            frame.render_widget(Paragraph::new("Hi").on_black(), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match read()? {
                Event::Key(event) => match event.modifiers {
                    KeyModifiers::CONTROL => {
                        if event.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
