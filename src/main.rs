mod app;
mod explorer;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

use crate::app::App;

fn main() -> Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut app = App::new()?;

    loop {
        app.draw()?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => app.navigate_up(),
                    KeyCode::Down => app.navigate_down(),
                    KeyCode::Left => app.go_to_parent()?,
                    KeyCode::Right => app.enter_directory()?,
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    terminal::disable_raw_mode()?;
    Ok(())
}
