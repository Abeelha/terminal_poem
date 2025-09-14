use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
    style::{Color, SetForegroundColor, ResetColor},
};
use std::io::{stdout, Write};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

struct PoemViewer {
    poems: Vec<(String, String)>,
    current_index: usize,
}

impl PoemViewer {
    fn new() -> Self {
        let mut poems = Vec::new();

        let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let exe_dir = exe_path.parent().unwrap_or(Path::new("."));

        let poems_dir = exe_dir.join("../../poems");

        if let Ok(entries) = fs::read_dir(&poems_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                        let filename = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();

                        if let Ok(content) = fs::read_to_string(&path) {
                            poems.push((filename, content));
                        }
                    }
                }
            }
        }

        poems.sort_by(|a, b| a.0.cmp(&b.0));

        PoemViewer {
            poems,
            current_index: 0,
        }
    }


    fn display(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), Clear(ClearType::Purge)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

        if self.poems.is_empty() {
            println!("No poems found in ./poems directory");
            return;
        }

        execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();
        println!("{}", self.poems[self.current_index].1);
        execute!(stdout(), ResetColor).unwrap();

        println!();

        execute!(stdout(), SetForegroundColor(Color::DarkGrey)).unwrap();
        println!("=======================================");
        println!(" ← → Switch poems | Q to quit");
        println!(" {} ({}/{})",
            self.poems[self.current_index].0.to_uppercase(),
            self.current_index + 1,
            self.poems.len()
        );
        println!("=======================================");
        execute!(stdout(), ResetColor).unwrap();

        stdout().flush().unwrap();
    }

    fn next_poem(&mut self) {
        if !self.poems.is_empty() {
            self.current_index = (self.current_index + 1) % self.poems.len();
        }
    }

    fn prev_poem(&mut self) {
        if !self.poems.is_empty() {
            if self.current_index == 0 {
                self.current_index = self.poems.len() - 1;
            } else {
                self.current_index -= 1;
            }
        }
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut viewer = PoemViewer::new();
    let mut stdout = stdout();

    viewer.display();

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                    KeyCode::Left => {
                        viewer.prev_poem();
                        viewer.display();
                    }
                    KeyCode::Right => {
                        viewer.next_poem();
                        viewer.display();
                    }
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    disable_raw_mode()?;

    Ok(())
}