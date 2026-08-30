use ratatui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct SearchBox<'a> {
    pub textarea: TextArea<'a>,
}

impl<'a> SearchBox<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}