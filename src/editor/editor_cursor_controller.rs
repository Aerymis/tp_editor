use std::cmp;

use crossterm::event::KeyCode;

use super::editor_rows::EditorRows;

pub struct CursorController {
    pub x: usize,
    pub y: usize,
    screen_columns: usize,
    screen_rows: usize,
    pub row_offset: usize,
    pub column_offset: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            x: 0,
            y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let number_of_rows = editor_rows.number_of_rows();
        match direction {
            KeyCode::Up => {
                self.y = self.y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.x != 0 {
                    self.x -= 1;
                } else if self.y > 0 {
                    self.y -= 1;
                    self.x = editor_rows.get_row(self.y).row_content.len();
                }
            }
            KeyCode::Down => {
                if self.y < number_of_rows {
                    self.y = self.y + 1;
                }
            }
            KeyCode::Right => {
                if self.y < number_of_rows {
                    match self.x.cmp(&editor_rows.get_row(self.y).row_content.len()) {
                        cmp::Ordering::Less => self.x += 1,
                        cmp::Ordering::Equal => {
                            self.y += 1;
                            self.x = 0;
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::End => {
                if self.y < number_of_rows {
                    self.x = editor_rows.get_row(self.y).row_content.len()
                }
            }
            KeyCode::Home => self.x = 0,
            _ => unimplemented!(),
        }
        let row_len = if self.y < number_of_rows {
            editor_rows.get_row(self.y).row_content.len()
        } else {
            0
        };
        self.x = cmp::min(self.x, row_len);
    }

    pub fn scroll(&mut self) {
        self.row_offset = cmp::min(self.row_offset, self.y);
        if self.y >= self.row_offset + self.screen_rows {
            self.row_offset = self.y - self.screen_rows + 1;
        }

        self.column_offset = cmp::min(self.column_offset, self.x);
        if self.x >= self.column_offset + self.screen_columns {
            self.column_offset = self.x - self.screen_columns + 1;
        }
    }
}
