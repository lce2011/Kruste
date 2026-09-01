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
    settings: Settings,
}

#[derive(Debug, Deserialize, Default)]
pub struct Lines {
    pub linenumber_fg: String,
    pub linenumber_bg: String,
    pub cursorline_fg: String,
    pub cursorline_bg: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Search {
    pub text: String,
    pub background: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Colors {
    pub text: String,
    pub background: String,
    pub border: String,
    pub lines: Lines,
    pub search: Search,
}

#[derive(Debug, Deserialize, Default)]
pub struct Cursorline {
    pub modifier: String
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub cursorline: Cursorline,
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
    
    pub fn read_linenumber_fg(&mut self) -> String {
        self.lines.linenumber_fg.clone()
    }

    pub fn read_linenumber_bg(&mut self) -> String {
        self.lines.linenumber_bg.clone()
    }

    pub fn read_cursorline_fg(&mut self) -> String {
        self.lines.cursorline_fg.clone()
    }

    pub fn read_cursorline_bg(&mut self) -> String {
        self.lines.cursorline_bg.clone()
    }

    pub fn read_search_fg(&mut self) -> String {
        self.search.text.clone()
    }

    pub fn read_search_bg(&mut self) -> String {
        self.search.background.clone()
    }
}

impl Cursorline {
    pub fn read_modifier(&mut self) -> String {
        self.modifier.clone()
    }
}

pub fn read_and_extract_colors(json_file: String) -> Colors {
    let json_data = read_to_string(json_file).unwrap();
    let json_root: Root = serde_json::from_str(&json_data).unwrap();

    let text_color = json_root.theme.colors.text;
    let bg_color = json_root.theme.colors.background;
    let border_color = json_root.theme.colors.border;

    let linenumber_fg_color = json_root.theme.colors.lines.linenumber_fg;
    let linenumber_bg_color = json_root.theme.colors.lines.linenumber_bg;
    let cursorline_fg_color = json_root.theme.colors.lines.cursorline_fg;
    let cursorline_bg_color = json_root.theme.colors.lines.cursorline_bg;

    let search_fg_color = json_root.theme.colors.search.text;
    let search_bg_color = json_root.theme.colors.search.background;

    Colors {
        text: text_color,
        background: bg_color,
        border: border_color,
        lines: Lines {
            linenumber_fg: linenumber_fg_color,
            linenumber_bg: linenumber_bg_color,
            cursorline_fg: cursorline_fg_color,
            cursorline_bg: cursorline_bg_color,
        },
        search: Search {
            text: search_fg_color,
            background: search_bg_color,
        },
    }
}

pub fn read_and_extract_settings(json_file: String) -> Settings {
    let json_data = read_to_string(json_file).unwrap();
    let json_root: Root = serde_json::from_str(&json_data).unwrap();

    let cursorline_modifier = json_root.theme.settings.cursorline.modifier;

    Settings {
        cursorline: Cursorline {
            modifier: cursorline_modifier,
        },
    }
}

pub fn parse_hex_color(hex: String) -> Color {
    let hex = hex.trim_start_matches('#');
        
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    Color::Rgb(r, g, b)
}