use std::fs::read_to_string;

use ratatui::style::Color::self;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Root {
    theme: Theme,
}

#[derive(Debug, Deserialize)]
struct Theme {
    colors: Colors,
}

#[derive(Debug, Deserialize, Default)]
pub struct Colors {
    pub text: String,
    pub background: String,
    pub border: String,
}

impl Colors {
    pub fn read_text(&mut self) -> String {
        self.text.clone()
    }

    pub fn read_bg(&mut self) -> String {
        self.background.clone()
    }

    pub fn read_border(&mut self) -> String {
        self.border.clone()
    }
}

pub fn read_and_extract_json(json_file: String) -> Colors {
    let json_data = read_to_string(json_file).unwrap();
    let json_root: Root = serde_json::from_str(&json_data).unwrap();

    let text_color = json_root.theme.colors.text;
    let bg_color = json_root.theme.colors.background;
    let border = json_root.theme.colors.border;

    Colors {
        text: text_color,
        background: bg_color,
        border: border
    }
}

pub fn parse_hex_color(hex: String) -> Color {
    let hex = hex.trim_start_matches('#');
        
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    Color::Rgb(r, g, b)
}