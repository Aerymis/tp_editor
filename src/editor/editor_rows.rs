use std::{env, fs, path::Path};

const TAB_SIZE: usize = 8;

pub struct Row {
    pub row_content: Box<str>,
    render: String,
}

impl Row {
    fn new(row_content: Box<str>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
}

pub struct EditorRows {
    row_contents: Vec<Row>,
}

impl EditorRows {
    pub fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_contents: Vec::new(),
            },
            Some(file) => Self::from_file(file.as_ref()),
        }
    }

    fn from_file(file: &Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            row_contents: file_contents
                .lines()
                .map(|line| {
                    let mut row = Row::new(line.into(), String::new());
                    Self::render_row(&mut row);
                    row
                })
                .collect(),
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    fn render_row(row: &mut Row) {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_SIZE } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_SIZE != 0 {
                    row.render.push(' ');
                    index += 1
                }
            } else {
                row.render.push(c);
            }
        })
    }

    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }
}
