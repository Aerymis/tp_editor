use std::{io::{stdout, Write}, cmp};

use crossterm::{
    cursor,
    event::KeyCode,
    execute, queue,
    terminal::{self, ClearType},
};

use super::{editor_content::EditorContent, editor_rows::EditorRows};

pub struct Output {
    pub win_size: (usize, usize),
    pub content: EditorContent,
    pub cursor: CursorController,
    pub rows: EditorRows,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            content: EditorContent::new(),
            cursor: CursorController::new(win_size),
            rows: EditorRows::new(),
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor.move_cursor(direction);
    }

    pub fn clear_screen() -> crossterm::Result<()> {
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
            if row >= self.rows.number_of_rows() {
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
            } else {
                let len = cmp::min(self.rows.get_row().len(), screen_columns);
                self.content
                    .push_str(&self.rows.get_row()[..len])
            }
        }
    }
}

pub struct CursorController {
    x: usize,
    y: usize,
    screen_columns: usize,
    _screen_rows: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            x: 0,
            y: 0,
            screen_columns: win_size.0,
            _screen_rows: win_size.1,
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
