mod app;
mod tui;
mod ui;

use crate::app::App;
use crate::tui::Tui;

use std::env::{args, current_dir};
use std::fs::{File, read_to_string};
use std::io::Write;

use crossterm::event::KeyModifiers;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    crossterm::event::{Event, KeyCode, read},
};
use ratatui_textarea::{Input, Key};

fn main() -> color_eyre::Result<()> {
    let args: Vec<String> = args().collect();
    let cwd = current_dir().unwrap().to_string_lossy().to_string();
    
    let file_name = args[1].clone();
    let file_path = format!("{}/{}", cwd, file_name);
    let file_content = read_to_string(&file_path)?;

    let mut file  = File::create(&file_path)?;
    let mut app = App::new(file_name, file_path, &file_content);
    
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    while !app.should_quit {
        let current_editor_content = app.textarea.lines().join("\n");

        if app.ask_save {
            tui.draw_ask_save()?;

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('n') => app.quit(),
                    KeyCode::Char('y') => {
                        file.write_all(format!("{}", current_editor_content).as_bytes())?;
                        app.quit();
                    }
                    _ => {}
                }
            }
        } else {
            tui.draw_editor(&mut app)?;

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Esc  => {
                        if file_content != app.textarea.lines().join("\n") {
                            app.ask_save = true;
                        } else {
                            app.quit();
                        }
                    }
                    KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
                        app.textarea.input(Input {
                            key: Key::Char(c),
                            ctrl: false,
                            alt: false,
                            shift: false,
                        });
                    }
                    _ => {
                        app.textarea.input(key);
                    }
                }
            }
        }
    }

    tui.exit()?;
    Ok(())
}
