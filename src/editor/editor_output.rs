use std::{
    cmp,
    io::{stdout, Write},
};

use crossterm::{
    cursor,
    event::KeyCode,
    execute, queue,
    terminal::{self, ClearType},
};

use super::{
    editor_content::EditorContent, editor_cursor_controller::CursorController,
    editor_rows::EditorRows,
};

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
        self.cursor.move_cursor(direction, &self.rows);
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        self.cursor.scroll();
        queue!(self.content, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor.x - self.cursor.column_offset;
        let cursor_y = self.cursor.y - self.cursor.row_offset;
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
            let file_row = row + self.cursor.row_offset;
            if file_row >= self.rows.number_of_rows() {
                if self.rows.number_of_rows() == 0 && row == screen_rows / 3 {
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
            } else {
                let row = self.rows.get_render(file_row);
                let column_offset = self.cursor.column_offset;
                let len = cmp::min(row.len().saturating_sub(column_offset), screen_columns);
                let start = if len == 0 { 0 } else { column_offset };
                self.content.push_str(&row[start..start + len]);
            }

            queue!(self.content, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if row < screen_rows - 1 {
                self.content.push_str("\r\n")
            }
        }
    }
}
