use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, queue, terminal};
use std::io::{stdout, Write};
use std::time::Duration;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

pub struct Keyboard;

impl Keyboard {
    fn new() -> Self {
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

pub struct CursorController {
    x: usize,
    y: usize,
    screen_columns: usize,
    screen_rows: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            x: 0,
            y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.y = self.y.saturating_sub(1);
            }
            KeyCode::Left => {
                self.x = self.x.saturating_sub(1);
            }
            KeyCode::Down => {
                self.y = self.y.saturating_add(1);
            }
            KeyCode::Right => {
                self.x = self.x.saturating_add(1);
            }
            KeyCode::End => self.x = self.screen_columns - 1,
            KeyCode::Home => self.x = 0,
            _ => unimplemented!(),
        }
    }
}

pub struct Output {
    win_size: (usize, usize),
    content: EditorContent,
    cursor: CursorController,
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            content: EditorContent::new(),
            cursor: CursorController::new(win_size),
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor.move_cursor(direction);
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.content, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor.x;
        let cursor_y = self.cursor.y;
        queue!(
            self.content,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.content.flush()
    }

    pub fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for row in 0..screen_rows {
            if row == screen_rows / 8 {
                let mut welcome = format!("Twin Planets Editor");
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns)
                }

                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.content.push('~');
                    padding -= 1
                }

                (0..padding).for_each(|_| self.content.push(' '));
                self.content.push_str(&welcome)
            } else {
                self.content.push('~');
            }

            queue!(self.content, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if row < screen_rows - 1 {
                self.content.push_str("\r\n")
            }
            stdout().flush().unwrap();
        }
    }
}

pub struct EditorContent {
    content: String,
}

impl EditorContent {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.content.push(c)
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl std::io::Write for EditorContent {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(std::io::ErrorKind::WriteZero.into()),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

pub struct Editor {
    pub keyboard: Keyboard,
    pub output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mock_main() {
        while Editor::new().run().unwrap() {}
        assert_eq!(true, true);
    }
}
