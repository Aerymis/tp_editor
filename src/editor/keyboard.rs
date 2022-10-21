use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Self
    }
    pub fn read_keypress(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                };
            }
        }
    }
}
