use std::{fs::File, io::Write};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_textarea::{Input, Key, TextArea};

use crate::json::{Colors, Settings};

#[derive(Debug, Default)]
pub struct App<'a> {
    pub file_name: String,
    pub file_path: String,
    pub should_quit: bool,
    pub ask_save: bool,
    pub search_overlay: bool,
    pub textarea: TextArea<'a>,
    pub colors: Colors,
    pub settings: Settings,
}

impl<'a> App<'a> {
    /* PUBLIC */
    pub fn new(file_name: String, file_path: String, file_content: &String) -> Self {
        Self {
            file_name: file_name,
            file_path: file_path,
            textarea: TextArea::from(file_content.lines()),
            ..Default::default()
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn input(&mut self, file: &mut File, file_content: String, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                if file_content != self.textarea.lines().join("\n") {
                    self.ask_save = true;
                } else {
                    let _ = file.write_all(format!("{}", file_content).as_bytes());
                    self.quit();
                }
            }
            KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
                self.textarea.input(Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
            }
            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.search_overlay = true;
            }
            _ => {
                self.textarea.input(key);
            }
        }
    }
}