use crate::editor::Document;
use crossterm::terminal;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Document::clear_screen().expect("Error");
    }
}
