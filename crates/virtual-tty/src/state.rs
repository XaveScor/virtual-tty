use crate::buffer::Buffer;
use crate::cursor::Cursor;

/// Unified state structure that combines buffer and cursor data
/// This replaces the previous dual-mutex approach with a single mutex
pub struct TtyState {
    pub buffer: Buffer,
    pub cursor: Cursor,
}

impl TtyState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            cursor: Cursor::new(),
        }
    }

    pub fn clear(&mut self, width: usize, height: usize) {
        self.buffer.clear();
        self.cursor.set_position(0, 0, height, width);
    }

    pub fn get_snapshot(&self) -> String {
        self.buffer.get_snapshot()
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor.get_position()
    }
}
