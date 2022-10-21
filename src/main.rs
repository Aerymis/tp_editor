use crossterm::terminal;
use tp_editor_core::{CleanUp, Editor};

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    while Editor::new().run()? {}
    Ok(())
}
