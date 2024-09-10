#[derive(Default)]
pub struct TextView {
    pub visible_lines: usize,          // view中可见的行数
    pub scroll_offset: (usize, usize), // 滚动偏移（行号,列号）
}

impl TextView {}
