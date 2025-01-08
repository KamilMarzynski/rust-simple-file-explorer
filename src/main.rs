use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use std::{
    fs,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

struct FileExplorer {
    current_path: PathBuf,
    entries: Vec<PathBuf>,
    selected_index: usize,
}

impl FileExplorer {
    fn new() -> Result<Self> {
        let current_path = std::env::current_dir()?;
        let mut explorer = Self {
            current_path,
            entries: Vec::new(),
            selected_index: 0,
        };
        explorer.update_entries()?;
        Ok(explorer)
    }

    fn update_entries(&mut self) -> Result<()> {
        self.entries.clear();
        for entry in fs::read_dir(&self.current_path)? {
            let entry = entry?;
            self.entries.push(entry.path());
        }
        self.entries.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a
                    .file_name()
                    .unwrap_or_default()
                    .cmp(&b.file_name().unwrap_or_default()),
            }
        });
        self.selected_index = self
            .selected_index
            .min(self.entries.len().saturating_sub(1));
        Ok(())
    }

    fn draw(&self) -> Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Print current directory
        println!("Current directory: {}", self.current_path.display());

        for (index, entry) in self.entries.iter().enumerate() {
            let prefix = if index == self.selected_index {
                ">"
            } else {
                " "
            };
            let name = entry
                .file_name()
                .context("Invalid filename")?
                .to_string_lossy();
            let display = if entry.is_dir() {
                format!("{} ?? {}/", prefix, name)
            } else {
                format!("{} ?? {}", prefix, name)
            };
            println!("{}", display);
        }
        Ok(())
    }

    fn navigate_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn navigate_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    fn enter_directory(&mut self) -> Result<()> {
        if let Some(selected) = self.entries.get(self.selected_index) {
            if selected.is_dir() {
                self.current_path = selected.clone();
                self.selected_index = 0;
                self.update_entries()?;
            }
        }
        Ok(())
    }

    fn go_to_parent(&mut self) -> Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.selected_index = 0;
            self.update_entries()?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut explorer = FileExplorer::new()?;

    loop {
        // Draw the current state
        explorer.draw()?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => explorer.navigate_up(),
                    KeyCode::Down => explorer.navigate_down(),
                    KeyCode::Left => explorer.go_to_parent()?,
                    KeyCode::Right => explorer.enter_directory()?,
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
