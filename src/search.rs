use crate::app::App;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_textarea::{Input, Key, TextArea};

#[derive(Debug, Default)]
pub struct SearchBox<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> SearchBox<'a> {
    /* PUBLIC */
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn input(&mut self, app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                app.search_overlay = false;
            }
            KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
                self.textarea.input(Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
            }
            _ => {
                self.textarea.input(key);
            }
        }
    }
}