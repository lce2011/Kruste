use crate::{app::App, json::parse_hex_color};

use ratatui::{
    Frame, layout::{Alignment, Constraint}, style::{Modifier, Style, Stylize}, widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

pub fn render_editor(app: &mut App, frame: &mut Frame) {
    let text_rgb_color = parse_hex_color(app.colors.read_text());
    let bg_rgb_color = parse_hex_color(app.colors.read_bg());
    let border_rgb_color = parse_hex_color(app.colors.read_border());

    let linenumber_fg_rgb_color = parse_hex_color(app.colors.read_linenumber_fg());
    let linenumber_bg_rgb_color = parse_hex_color(app.colors.read_linenumber_bg());
    let cursorline_fg_rgb_color = parse_hex_color(app.colors.read_cursorline_fg());
    let cursorline_bg_rgb_color = parse_hex_color(app.colors.read_cursorline_bg());

    let cursorline_modifier = app.settings.cursorline.read_modifier();

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

    let linenumber_style = Style::default()
        .fg(linenumber_fg_rgb_color)
        .bg(linenumber_bg_rgb_color);
    let cursorline_style = Style::default()
        .add_modifier(match cursorline_modifier.to_uppercase().as_str() {
            "BOLD" => Modifier::BOLD,
            "DIM" => Modifier::DIM,
            "ITALIC" => Modifier::ITALIC,
            "UNDERLINED" => Modifier::UNDERLINED,
            "SLOW_BLINK" => Modifier::SLOW_BLINK,
            "RAPID_BLINK" => Modifier::RAPID_BLINK,
            "REVERSED" => Modifier::REVERSED,
            "HIDDEN" => Modifier::HIDDEN,
            "CROSSED_OUT" => Modifier::CROSSED_OUT,
            _ => Modifier::empty()
        })
        .fg(cursorline_fg_rgb_color)
        .bg(cursorline_bg_rgb_color);

    app.textarea.set_line_number_style(linenumber_style);
    app.textarea.set_cursor_line_style(cursorline_style);
    app.textarea.set_block(block);

    frame.render_widget(&app.textarea, frame.area());
}

pub fn render_ask_save(app: &mut App, frame: &mut Frame) {
    let text_rgb_color = parse_hex_color(app.colors.read_text());
    let bg_rgb_color = parse_hex_color(app.colors.read_bg());
    let border_rgb_color = parse_hex_color(app.colors.read_border());

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