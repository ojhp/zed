use rustyline::Editor;

use crate::read::{Read, ReadResult};

pub struct ReplTextReader {
    editor: Editor<()>,
}

impl ReplTextReader {
    pub fn new() -> Self {
        let editor = Editor::new();

        ReplTextReader { editor }
    }
}

impl Read for ReplTextReader {
    type Output = String;

    fn read(&mut self) -> ReadResult<String> {
        fn match_braces(text: &str) -> bool {
            let mut level = 0;

            for ch in text.chars() {
                if ch == '(' {
                    level += 1;
                } else if ch == ')' {
                    level -= 1;
                }
            }

            level <= 0
        }

        let mut text = self.editor.readline("> ")?;

        while !match_braces(&text) {
            let line = self.editor.readline("...> ")?;

            text.push('\n');
            text.push_str(&line);
        }

        Ok(text)
    }
}
