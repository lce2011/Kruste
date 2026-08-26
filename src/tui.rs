use std::{io, panic};

use ratatui::crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    event::{DisableMouseCapture, EnableMouseCapture}, execute,
};

use crate::app::App;
use crate::ui;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    /* PUBLIC */
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset the terminal!");
            panic_hook(panic);
        }));

        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw_editor(&mut self, app: &mut App) -> color_eyre::Result<()> {
        self.terminal.draw(|frame| ui::render_editor(app, frame))?;
        Ok(())
    }

    pub fn draw_ask_save(&mut self) -> color_eyre::Result<()> {
        self.terminal.draw(|frame| ui::render_ask_save(frame))?;
        Ok(())
    }

    pub fn exit(&mut self) -> color_eyre::Result<()> {
        Self::reset()?;
        Ok(())
    }

    /* PRIVATE */
    fn reset() -> color_eyre::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}