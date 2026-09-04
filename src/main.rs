mod app;
mod tui;
mod ui;
mod json;
mod search;
mod helpers;

use crate::app::App;
use crate::helpers::set_gefault_settings;
use crate::json::{read_and_extract_colors, read_and_extract_settings};
use crate::search::SearchBox;
use crate::tui::Tui;

use std::env::{args, current_dir, var};
use std::fs::{File, read_to_string, exists, OpenOptions};
use std::io::Write;

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    crossterm::event::{Event, KeyCode, read},
};

fn main() -> color_eyre::Result<()> {
    let args: Vec<String> = args().collect();
    let cwd = current_dir().unwrap().to_string_lossy().to_string();
    
    let file_name = args[1].clone();
    let file_path = format!("{}/{}", cwd, file_name);

    let mut file_content = "".to_string();
    let mut file: File;

    if exists(&file_path)? {
        file_content = read_to_string(&file_path)?;
        file = OpenOptions::new()
            .write(true)
            .create(false)
            .open(&file_path)?;
    } else {
        file = File::create(&file_path)?;
    }
    
    let mut app = App::new(file_name, file_path, &file_content);
    let mut searchbox = SearchBox::new();
    
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
            set_gefault_settings(&mut app);
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
        } else if app.search_overlay {
            tui.draw_search_overlay(&mut app, &mut searchbox)?;

            if let Event::Key(key) = read()? {
                searchbox.input(&mut app, key);
            }
        } else {
            tui.draw_editor(&mut app)?;

            if let Event::Key(key) = read()? {
                app.input(&mut file, file_content.clone(), key);
            }
        }
    }

    tui.exit()?;
    Ok(())
}
