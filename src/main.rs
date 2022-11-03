use crossterm::terminal;
use tp_editor_core::{editor::utils::CleanUp, init_editor};

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    init_editor()
}
