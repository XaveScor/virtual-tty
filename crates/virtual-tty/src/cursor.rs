pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn move_up(&mut self, n: usize) {
        self.row = self.row.saturating_sub(n);
    }

    pub fn move_down(&mut self, n: usize, max_height: usize) {
        self.row = (self.row + n).min(max_height - 1);
    }

    pub fn move_forward(&mut self, n: usize, max_width: usize) {
        self.col = (self.col + n).min(max_width - 1);
    }

    pub fn move_back(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    pub fn set_position(&mut self, row: usize, col: usize, max_height: usize, max_width: usize) {
        self.row = row.min(max_height - 1);
        self.col = col.min(max_width - 1);
    }

    pub fn carriage_return(&mut self) {
        self.col = 0;
    }

    pub fn newline(&mut self, max_height: usize) -> bool {
        self.col = 0;
        self.row += 1;
        if self.row >= max_height {
            self.row = max_height - 1;
            true // Indicates scrolling is needed
        } else {
            false
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn advance(&mut self, max_width: usize, max_height: usize) -> bool {
        self.col += 1;
        if self.col >= max_width {
            self.col = 0;
            self.row += 1;
            if self.row >= max_height {
                self.row = max_height - 1;
                true // Indicates scrolling is needed
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}
