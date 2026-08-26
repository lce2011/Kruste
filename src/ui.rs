use crate::app::App;


use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};

pub fn render_editor(app: &mut App, frame: &mut Frame) {
    let block = Block::default()
                .title(format!(" {} ", app.file_name))
                .title_alignment(Alignment::Center)
                .title_bottom(format!(" {} ", app.file_path))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White))
                .padding(Padding::uniform(0));
    app.textarea.set_block(block);

    frame.render_widget(&app.textarea, frame.area());
}

pub fn render_ask_save(frame: &mut Frame) {
    let block = Block::default()
        .title(" Save to file? ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .padding(Padding::uniform(0));
    let centered_area = frame.area()
        .centered(
            Constraint::Percentage(60), 
            Constraint::Percentage(20));
    let paragraph = Paragraph::new("<'y' Yes  |  'n' No")
        .alignment(Alignment::Center)
        .block(block);

    frame.render_widget(Clear, centered_area);
    frame.render_widget(paragraph, centered_area);
}