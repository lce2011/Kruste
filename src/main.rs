mod app;
mod tui;
mod ui;
mod json;

use crate::app::App;
use crate::json::{Colors, Cursorline, Lines, Settings, read_and_extract_colors, read_and_extract_settings};
use crate::tui::Tui;

use std::env::{args, current_dir, var};
use std::fs::{File, read_to_string, exists};
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

    let mut file_content = "".to_string();
    let mut file: File;

    if exists(&file_path)? {
        file_content = read_to_string(&file_path)?;
        file  = File::create(&file_path)?;
    } else {
        file  = File::create(&file_path)?;
    }
    
    let mut app = App::new(file_name, file_path, &file_content);
    
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);

    let config_path_variable = "KRUSTE_CONFIG";
    match var(config_path_variable) {
        Ok(value) => {
            println!("Found Kruste config at {}.", value);
            let config_file = value;
            app.colors = read_and_extract_colors(config_file.clone());
            app.settings = read_and_extract_settings(config_file.clone());
        }
        Err(e) => {
            println!("Error: Couldn't find Kruste config!\n{}: {}", config_path_variable, e);
            println!("Using default config for now.");
            app.colors = Colors {
                text: "#FFFFFF".to_string(),
                background: "#000000".to_string(),
                border: "#FFFFFF".to_string(),
                lines: Lines {
                    linenumber_fg: "#FFFFFF".to_string(),
                    linenumber_bg: "#808080".to_string(),
                    cursorline_fg: "#FFFFFF".to_string(),
                    cursorline_bg: "#4b4b4b".to_string(),
                },
            };
            app.settings = Settings {
                cursorline: Cursorline {
                    modifier: "".to_string(),
                },
            }
        }
    }

    tui.enter()?;

    while !app.should_quit {
        let current_editor_content = app.textarea.lines().join("\n");

        if app.ask_save {
            tui.draw_ask_save(&mut app)?;

            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('e') => app.ask_save = false,
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
                            file.write_all(format!("{}", file_content).as_bytes())?;
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
