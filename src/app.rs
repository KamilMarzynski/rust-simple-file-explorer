use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::io::stdout;

use crate::explorer::FileExplorer;

pub struct App {
    explorer: FileExplorer,
}

impl App {
    pub fn new() -> Result<Self> {
        let explorer = FileExplorer::new()?;

        Ok(Self { explorer })
    }

    pub fn draw(&self) -> Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Print current directory
        println!(
            "Current directory: {}",
            self.explorer.current_path.display()
        );
        println!("Commands: Arrow keys to navigate, 't' to toggle tree view, 'q' to quit");
        let size = terminal::size()?;

        self.explorer.draw()?;

        Ok(())
    }

    pub fn navigate_up(&mut self) {
        self.explorer.navigate_up();
    }

    pub fn navigate_down(&mut self) {
        self.explorer.navigate_down();
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        self.explorer.enter_directory()?;
        Ok(())
    }

    pub fn go_to_parent(&mut self) -> Result<()> {
        self.explorer.go_to_parent()?;
        Ok(())
    }
}
