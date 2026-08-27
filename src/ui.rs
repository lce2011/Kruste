use crate::{app::App, json::parse_hex_color};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

pub fn render_editor(app: &mut App, frame: &mut Frame) {
    let text_hex_color = app.colors.read_text();
    let bg_hex_color = app.colors.read_bg();
    let border_hex_color = app.colors.read_border();

    let text_rgb_color = parse_hex_color(text_hex_color);
    let bg_rgb_color = parse_hex_color(bg_hex_color);
    let border_rgb_color = parse_hex_color(border_hex_color);

    let block = Block::default()
                .title(format!(" {} ", app.file_name))
                .title_alignment(Alignment::Center)
                .title_bottom(format!(
                    " {}  |  'Esc' Exit & Save  |  'E' When in Esc, go back to editor ",
                    app.file_path))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_rgb_color))
                .padding(Padding::uniform(0))
                .bg(bg_rgb_color)
                .fg(text_rgb_color);
    app.textarea.set_block(block);

    frame.render_widget(&app.textarea, frame.area());
}

pub fn render_ask_save(app: &mut App, frame: &mut Frame) {
    let text_hex_color = app.colors.read_text();
    let bg_hex_color = app.colors.read_bg();
    let border_hex_color = app.colors.read_border();

    let text_rgb_color = parse_hex_color(text_hex_color);
    let bg_rgb_color = parse_hex_color(bg_hex_color);
    let border_rgb_color = parse_hex_color(border_hex_color);

    let block = Block::default()
        .title(" Save to file? ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_rgb_color))
        .padding(Padding::uniform(0))
        .bg(bg_rgb_color)
        .fg(text_rgb_color);
    let centered_area = frame.area()
        .centered(
            Constraint::Percentage(15),
            Constraint::Percentage(8));
    let paragraph = Paragraph::new("'y' Yes  |  'n' No")
        .alignment(Alignment::Center)
        .block(block);

    frame.render_widget(Clear, centered_area);
    frame.render_widget(paragraph, centered_area);
}