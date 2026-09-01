use crate::{app::App, json::parse_hex_color};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Style;
use ratatui_textarea::{Input, Key, TextArea};

#[derive(Debug, Default)]
pub struct SearchBox<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> SearchBox<'a> {
    /* PUBLIC */
    pub fn new() -> Self {
        Self {
            textarea: TextArea::default(),
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

        let _ = self.search(app);
    }

    /* PRIVATE */
    fn search(&mut self, app: &mut App) -> color_eyre::Result<()> {
        let text_rgb_color = parse_hex_color(app.colors.read_search_fg());
        let bg_rgb_color = parse_hex_color(app.colors.read_search_bg());
        
        let style = Style::default()
            .fg(text_rgb_color)
            .bg(bg_rgb_color);


        let text_to_search = &self.textarea.lines()[0];

        app.textarea.set_search_pattern(text_to_search)?;
        app.textarea.set_search_style(style);

        Ok(())
    }
}