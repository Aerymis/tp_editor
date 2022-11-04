use crate::editor::editor_document::Document;
use crate::editor::keyboard::Keyboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cmp;

pub mod editor_content;
pub mod editor_cursor_controller;
pub mod editor_document;
pub mod editor_rows;
pub mod keyboard;
pub mod utils;

pub struct Editor {
    pub keyboard: Keyboard,
    pub document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard,
            document: Document::new(),
        }
    }
    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.document.refresh_screen()?;
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
            } => self.document.move_cursor(direction),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if matches!(val, KeyCode::PageUp) {
                    self.document.cursor.y = self.document.cursor.row_offset;
                } else {
                    self.document.cursor.y = cmp::min(
                        self.document.win_size.1 + self.document.cursor.row_offset - 1,
                        self.document.rows.number_of_rows(),
                    );
                }

                (0..self.document.win_size.1).for_each(|_| {
                    self.document.move_cursor(if matches!(val, KeyCode::PageUp) {
                        KeyCode::Up
                    } else {
                        KeyCode::Down
                    });
                })
            }
            _ => {}
        }

        Ok(true)
    }
}
