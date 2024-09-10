use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_position: (u16, u16),
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            cursor_position: (0, 0),
        }
    }

    pub fn open_file(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        self.lines.clear();

        for line in reader.lines() {
            match line {
                Ok(content) => {
                    self.lines.push(content);
                }
                Err(e) => return Err(e),
            }
        }

        self.cursor_position.0 = 0;
        self.cursor_position.1 = 0;

        Ok(())
    }
}
