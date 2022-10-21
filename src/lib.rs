pub mod editor;

use editor::{Editor, utils::CleanUp};

pub fn init_editor() -> crossterm::Result<()> {
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}

pub fn clean_up() -> CleanUp {
    CleanUp
}

#[cfg(test)]
mod tests {
    use crate::editor::Editor;

    #[test]
    fn mock_main() {
        while Editor::new().run().unwrap() {}
        assert_eq!(true, true);
    }
}
