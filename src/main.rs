use crossterm::terminal;
use tp_editor_core::{init_editor, clean_up};

fn main() -> crossterm::Result<()> {
    let _clean_up = clean_up();
    terminal::enable_raw_mode()?;
    init_editor()
}
