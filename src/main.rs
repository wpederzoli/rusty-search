use std::io::{stdout, Result};

use app::App;
use crossterm::{
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};

mod app;

fn main() -> Result<()> {
    let app = App::new();

    app.run();

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
