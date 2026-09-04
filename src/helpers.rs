use crate::app::App;
use crate::json::{Colors, Cursorline, Linenumbers, Lines, Search, Settings};

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/* CONFIG HELPERS */
pub fn set_gefault_settings(app: &mut App) {
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
        search: Search {
            text: "#FFFFFF".to_string(),
            background: "#00FF00".to_string(),
        },
    };
    app.settings = Settings {
        cursorline: Cursorline {
            enabled: false,
            modifier: "".to_string(),
        },
        linenumbers: Linenumbers {
            enabled: false,
        }
    };
}

/* UI HELPERS */
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub fn top_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(0),
            Constraint::Length(percent_y),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}