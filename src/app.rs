use ratatui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct App<'a> {
    pub file_name: String,
    pub file_path: String,
    pub should_quit: bool,
    pub ask_save: bool,
    pub textarea: TextArea<'a>,
}

impl<'a> App<'a> {
    /* PUBLIC */
    pub fn new(file_name: String, file_path: String, file_content: &String) -> Self {
        Self {
            file_name: file_name,
            file_path: file_path,
            textarea: TextArea::from(file_content.lines()),
            ..Default::default()
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}