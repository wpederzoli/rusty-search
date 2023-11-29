use std::io::{stdout, Stdout};

use crossterm::{
    event::{self, read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new() -> Self {
        stdout()
            .execute(EnterAlternateScreen)
            .expect("Unable to enter alter terminal");
        enable_raw_mode().expect("Failed to enable raw mode");
        let mut terminal =
            Terminal::new(CrosstermBackend::new(stdout())).expect("Failed to create new terminal");
        terminal.clear().expect("Unable to clear terminal");

        App { terminal }
    }

    pub fn run(mut self) {
        loop {
            self.terminal
                .draw(|frame| {
                    let mut area = frame.size();
                    area.height = area.height / 2;
                    area.width = area.width / 2;
                    frame.render_widget(Paragraph::new("Hi"), area);
                })
                .expect("Failed to draw to terminal");

            if event::poll(std::time::Duration::from_millis(16)).expect("Failed to poll events") {
                match read().expect("Failed to read from events") {
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
    }
}
