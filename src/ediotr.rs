use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Add;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Print, PrintStyledContent, Stylize};
use crossterm::{cursor, queue, terminal};
use unicode_width::UnicodeWidthStr;

use crate::terminal::Terminal;
use crate::text_buffer::TextBuffer;
use crate::text_view::TextView;

pub struct Editor {
    text_buffer: TextBuffer,
    text_view: TextView,
    terminal: Terminal,
    is_exit: bool,
    file_name: String,
}

impl Editor {
    pub fn new() -> Result<Self, std::io::Error> {
        let args: Vec<String> = std::env::args().collect();
        let mut text_buffer = TextBuffer::new();

        let mut file_name = String::new();
        if args.len() > 1 {
            file_name = args[1].clone();
            text_buffer.open_file(&file_name)?;
        }

        Ok(Self {
            text_buffer: text_buffer,
            text_view: TextView::default(),
            terminal: Terminal::new()?,
            is_exit: false,
            file_name: file_name,
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            if let Err(error) = self.redraw_terminal() {
                return Err(error);
            }

            if self.is_exit {
                terminal::disable_raw_mode()?;
                self.reset_display()?;
                break;
            }

            if let Err(error) = self.process_key_map() {
                return Err(error);
            }
        }
        Ok(())
    }

    fn redraw_terminal(&mut self) -> Result<(), std::io::Error> {
        queue!(self.terminal._stdout, cursor::Hide)?;
        self.reset_display()?;
        self.render_line();
        self.render_status_bar();

        queue!(self.terminal._stdout, cursor::Show)?;
        queue!(
            self.terminal._stdout,
            cursor::MoveTo(
                self.text_buffer.cursor_position.0,
                self.text_buffer.cursor_position.1
            )
        )?;

        self.terminal._stdout.flush()?;

        Ok(())
    }

    fn reset_display(&mut self) -> Result<(), std::io::Error> {
        queue!(
            self.terminal._stdout,
            terminal::Clear(terminal::ClearType::All)
        )?;

        Ok(())
    }

    fn render_line(&mut self) {
        let ascii_art = vec![
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⣠⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣡⣾⣿⣿⣿⣿⣿⢿⣿⣿⣿⣿⣿⣿⣟⠻⣿⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⡿⢫⣷⣿⣿⣿⣿⣿⣿⣿⣾⣯⣿⡿⢧⡚⢷⣌⣽⣿⣿⣿⣿⣿⣶⡌⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⠇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣮⣇⣘⠿⢹⣿⣿⣿⣿⣿⣻⢿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⠀⢸⣿⣿⡇⣿⣿⣿⣿⣿⣿⣿⣿⡟⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣻⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⡇⠀⣬⠏⣿⡇⢻⣿⣿⣿⣿⣿⣿⣿⣷⣼⣿⣿⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢻⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⠀⠈⠁⠀⣿⡇⠘⡟⣿⣿⣿⣿⣿⣿⣿⣿⡏⠿⣿⣟⣿⣿⣿⣿⣿⣿⣿⣿⣇⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⡏⠀⠀⠐⠀⢻⣇⠀⠀⠹⣿⣿⣿⣿⣿⣩⡶⠼⠟⠻⠞⣿⡈⠻⣟⢻⣿⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⢿⠀⡆⠀⠘⢿⢻⡿⣿⣧⣷⢣⣶⡃⢀⣾⡆⡋⣧⠙⢿⣿⣿⣟⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⡥⠂⡐⠀⠁⠑⣾⣿⣿⣾⣿⣿⣿⡿⣷⣷⣿⣧⣾⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⡿⣿⣍⡴⠆⠀⠀⠀⠀⠀⠀⠀⠀⣼⣄⣀⣷⡄⣙⢿⣿⣿⣿⣿⣯⣶⣿⣿⢟⣾⣿⣿⢡⣿⣿⣿⣿⣿",
            "⣿⡏⣾⣿⣿⣿⣷⣦⠀⠀⠀⢀⡀⠀⠀⠠⣭⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⣡⣾⣿⣿⢏⣾⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⡴⠀⠀⠀⠀⠀⠠⠀⠰⣿⣿⣿⣷⣿⠿⠿⣿⣿⣭⡶⣫⠔⢻⢿⢇⣾⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⡿⢫⣽⠟⣋⠀⠀⠀⠀⣶⣦⠀⠀⠀⠈⠻⣿⣿⣿⣾⣿⣿⣿⣿⡿⣣⣿⣿⢸⣾⣿⣿⣿⣿⣿⣿⣿",
            "⡿⠛⣹⣶⣶⣶⣾⣿⣷⣦⣤⣤⣀⣀⠀⠀⠀⠀⠀⠀⠉⠛⠻⢿⣿⡿⠫⠾⠿⠋⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣀⡆⣠⢀⣴⣏⡀⠀⠀⠀⠉⠀⠀⢀⣠⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⠿⠛⠛⠛⠛⠛⠛⠻⢿⣿⣿⣿⣿⣯⣟⠷⢷⣿⡿⠋⠀⠀⠀⠀⣵⡀⢠⡿⠋⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⢿⣿⣿⠂⠀⠀⠀⠀⠀⢀⣽⣿⣿⣿⣿⣿⣿⣿⣍⠛⠿⣿⣿⣿⣿⣿⣿",
        ];

        for index in 0..self.terminal.size.height.saturating_sub(2) {
            let actual_index =
                index.saturating_add(self.text_view.scroll_offset.1.try_into().unwrap()) as usize;

            if let Some(text_line) = self.text_buffer.lines.get(actual_index) {
                let text = text_line
                    .clone()
                    .chars()
                    .skip(self.text_view.scroll_offset.0)
                    .take(self.terminal.size.width as usize)
                    .collect::<String>();

                queue!(
                    self.terminal._stdout,
                    cursor::MoveTo(0, index.try_into().unwrap()),
                    Print(&text)
                )
                .unwrap();
            } else if self.text_buffer.lines.is_empty()
                && index
                    == (self.terminal.size.height.saturating_div(3) - (ascii_art.len() as u16 / 3))
                        .into()
            {
                for message in ascii_art.clone().into_iter() {
                    self.render_message(message)
                }
            } else {
                self.render_empty_line(index as usize);
            }
        }
    }

    fn render_message(&mut self, message: &str) {
        let message_width = UnicodeWidthStr::width(message);
        let width = self
            .terminal
            .size
            .width
            .saturating_div(2)
            .saturating_sub((message_width / 2).try_into().unwrap());

        let padding = " ".repeat(width.into());
        let result = format!("{}{}", padding, message);

        println!("\r~{}", result);
    }

    fn render_empty_line(&mut self, y: usize) {
        queue!(
            self.terminal._stdout,
            cursor::MoveTo(0, y.try_into().unwrap()),
            Print("~"),
        )
        .unwrap();
    }

    fn render_status_bar(&mut self) {
        let x = self.terminal.size.width;
        let y = self.terminal.size.height;

        let padding = "-".repeat(x as usize);

        let cursor_position = format!(
            "size({},{}) | scroll_offset({},{}) | {} lines |Cursor: {}:{}",
            self.terminal.size.width,
            self.terminal.size.height,
            self.text_view.scroll_offset.0,
            self.text_view.scroll_offset.1,
            self.text_view.visible_lines.saturating_add(1),
            self.text_buffer.cursor_position.0,
            self.text_buffer.cursor_position.1
        );
        let cursor_position_len = cursor_position.len() + (cursor_position.len() / 10);

        queue!(
            self.terminal._stdout,
            cursor::MoveTo(0, y - 2),
            PrintStyledContent(padding.on_dark_grey()),
            cursor::MoveTo(4, y - 2),
            PrintStyledContent(self.file_name.clone().on_dark_grey()),
            cursor::MoveTo(x - cursor_position_len as u16, y - 2),
            PrintStyledContent(cursor_position.on_dark_grey()),
        )
        .unwrap();
    }

    fn process_key_map(&mut self) -> Result<(), std::io::Error> {
        match read()? {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.is_exit = true,
                (KeyCode::Char('n'), KeyModifiers::CONTROL) => self.scroll_down(),
                (KeyCode::Char('p'), KeyModifiers::CONTROL) => self.scroll_up(),
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.scroll_half_page_down(),
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.scroll_half_page_up(),
                (KeyCode::Char('b'), KeyModifiers::CONTROL) => self.scroll_left(),
                (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.scroll_right(),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.save_to_file()?,
                (KeyCode::Backspace, _) => self.delete_char(),
                (KeyCode::Char(c), _) => self.write_char(c),
                (KeyCode::Enter, _) => self.enter(),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn enter(&mut self) {
        let current_line_index = self.text_view.visible_lines;

        let mut current_line = match self.text_buffer.lines.get_mut(current_line_index) {
            Some(line) => line.clone(),
            None => return,
        };

        let cursor_pos = self.text_buffer.cursor_position.0 as usize;

        let (before_cursor, after_cursor) = current_line.split_at(cursor_pos);

        self.text_buffer.lines[current_line_index] = before_cursor.to_string();

        self.text_buffer
            .lines
            .insert(current_line_index + 1, after_cursor.to_string());

        self.text_buffer.cursor_position.0 = 0;
        self.text_buffer.cursor_position.1 += 1;

        self.text_view.visible_lines += 1;

        let visible_area = self.terminal.size.height.saturating_sub(2);
        if self.text_view.visible_lines >= self.text_buffer.lines.len() {
            self.text_view.visible_lines = self.text_buffer.lines.len() - 1;
        }

        if self.text_view.visible_lines >= visible_area as usize {
            self.text_view.scroll_offset.1 = self
                .text_view
                .visible_lines
                .saturating_sub(visible_area as usize);
        } else {
            self.text_view.scroll_offset.1 = 0;
        }
    }

    fn delete_char(&mut self) {
        let cursor_position = self.text_buffer.cursor_position.0;
        let scroll_offset = self.text_view.scroll_offset.0;

        if let Some(s) = self.text_buffer.lines.get_mut(self.text_view.visible_lines) {
            let index = cursor_position + scroll_offset as u16;
            let char_indices: Vec<(usize, char)> = s.char_indices().collect();

            if let Some((char_start, _)) = char_indices.get(index as usize) {
                s.remove(*char_start);
                self.scroll_left();
            }
        }

        if let Some(s) = self.text_buffer.lines.get_mut(self.text_view.visible_lines) {
            if s.is_empty() {
                self.text_buffer.lines.remove(self.text_view.visible_lines);
            }
        }
    }

    fn write_char(&mut self, c: char) {
        if let Some(text_line) = self.text_buffer.lines.get_mut(self.text_view.visible_lines) {
            let x = self.text_buffer.cursor_position.0 + self.text_view.scroll_offset.0 as u16;
            text_line.insert(x.into(), c);
            self.scroll_right();
        }
    }

    fn scroll_left(&mut self) {
        if self.text_buffer.cursor_position.0 > 0 {
            self.text_buffer.cursor_position.0 =
                self.text_buffer.cursor_position.0.saturating_sub(1);
            self.text_view.scroll_offset.0 = self.text_view.scroll_offset.0.saturating_sub(1);
        } else if self.text_buffer.cursor_position.1 > 0 {
            self.scroll_up();
            if let Some(prev_line_text) = self.text_buffer.lines.get(self.text_view.visible_lines) {
                self.text_buffer.cursor_position.0 = prev_line_text.len() as u16;
            }
        }
    }

    fn scroll_right(&mut self) {
        if self.text_buffer.cursor_position.0 > self.terminal.size.width - 2 {
            self.text_buffer.cursor_position.0 = self.terminal.size.width - 2;
            self.text_view.scroll_offset.0 = self.text_view.scroll_offset.0.saturating_add(1);
        }

        if let Some(current_line_text) =
            self.text_buffer.lines.get_mut(self.text_view.visible_lines)
        {
            if self.text_buffer.cursor_position.0 <= current_line_text.len() as u16 {
                self.text_buffer.cursor_position.0 =
                    self.text_buffer.cursor_position.0.saturating_add(1);
            }
            if self.text_buffer.cursor_position.0 > current_line_text.len() as u16
                || self.text_buffer.cursor_position.0 + self.text_view.scroll_offset.0 as u16
                    > current_line_text.len() as u16
            {
                self.scroll_down();
                self.text_buffer.cursor_position.0 = 0;
            }
        }
    }

    fn scroll_down(&mut self) {
        if self.text_buffer.cursor_position.1 < self.terminal.size.height.saturating_sub(3) {
            self.text_buffer.cursor_position.1 =
                self.text_buffer.cursor_position.1.saturating_add(1);
        } else {
            self.text_view.scroll_offset.1 = self.text_view.scroll_offset.1.saturating_add(1);
        }

        self.text_view.visible_lines =
            self.text_view.scroll_offset.1 + self.text_buffer.cursor_position.1 as usize;

        let visible_area = self.terminal.size.height.saturating_sub(2);

        if self.text_view.scroll_offset.1 + visible_area as usize > self.text_buffer.lines.len() {
            self.text_view.scroll_offset.1 = self
                .text_buffer
                .lines
                .len()
                .saturating_sub(visible_area as usize);
        }

        if let Some(next_line) = self.text_buffer.lines.get(self.text_view.visible_lines) {
            if let Some(_this_line) = self
                .text_buffer
                .lines
                .get(self.text_view.visible_lines.saturating_sub(1))
            {
                let current_cursor_x = self.text_buffer.cursor_position.0 as usize;
                let new_cursor_x = current_cursor_x.min(next_line.len());

                self.text_buffer.cursor_position.0 = new_cursor_x as u16;
            }
        }
    }

    fn scroll_up(&mut self) {
        if self.text_buffer.cursor_position.1 > 0 {
            self.text_buffer.cursor_position.1 =
                self.text_buffer.cursor_position.1.saturating_sub(1);
        } else {
            self.text_view.scroll_offset.1 = self.text_view.scroll_offset.1.saturating_sub(1);
        }
        self.text_view.visible_lines =
            self.text_view.scroll_offset.1 + self.text_buffer.cursor_position.1 as usize;
    }

    fn scroll_half_page_down(&mut self) {
        self.text_view.scroll_offset.1 = std::cmp::min(
            self.text_buffer.lines.len() as usize - (self.terminal.size.height as usize - 2),
            self.text_view
                .scroll_offset
                .1
                .saturating_add(self.terminal.size.height as usize),
        );
        self.text_view.visible_lines =
            self.text_view.scroll_offset.1 + self.text_buffer.cursor_position.1 as usize;
    }

    fn scroll_half_page_up(&mut self) {
        self.text_view.scroll_offset.1 = std::cmp::max(
            0,
            self.text_view
                .scroll_offset
                .1
                .saturating_sub(self.terminal.size.height as usize),
        );
        self.text_view.visible_lines =
            self.text_view.scroll_offset.1 + self.text_buffer.cursor_position.1 as usize;
    }

    pub fn save_to_file(&mut self) -> std::io::Result<()> {
        let file = File::create(self.file_name.clone())?;
        let mut writer = BufWriter::new(file);

        for line in self.text_buffer.lines.iter() {
            writeln!(writer, "{}", line)?;
        }
        writer.flush()?;
        terminal::disable_raw_mode()?;
        self.reset_display()?;
        self.is_exit = true;
        Ok(())
    }
}
