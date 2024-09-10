use std::io;

use crossterm::terminal;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new() -> Result<Self, std::io::Error> {
        let (width, height) = terminal::size()?;
        Ok(Self { width, height })
    }
}

pub struct Terminal {
    pub _stdout: std::io::Stdout,
    pub size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        terminal::enable_raw_mode()?;

        Ok(Self {
            _stdout: std::io::stdout(),
            size: Size::new()?,
        })
    }
}
