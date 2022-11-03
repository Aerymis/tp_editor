pub mod editor;

use editor::{Editor};

pub fn init_editor() -> crossterm::Result<()> {
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
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
