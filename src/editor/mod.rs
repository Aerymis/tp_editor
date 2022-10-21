use crate::editor::editor_output::Output;
use crate::editor::keyboard::Keyboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub mod editor_content;
pub mod editor_output;
pub mod editor_rows;
pub mod keyboard;
pub mod utils;

pub struct Editor {
    pub keyboard: Keyboard,
    pub output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard,
            output: Output::new(),
        }
    }
    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.keyboard.read_keypress()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End),
                modifiers: KeyModifiers::NONE,
                ..
            } => self.output.move_cursor(direction),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            } => (0..self.output.win_size.1).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),
            _ => {}
        }

        Ok(true)
    }
}
