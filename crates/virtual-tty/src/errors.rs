pub type Result<T> = std::result::Result<T, VirtualTtyError>;

#[derive(Debug)]
pub enum VirtualTtyError {
    InvalidEscapeSequence(String),
    CursorOutOfBounds { row: usize, col: usize },
    InvalidParameter(String),
}

impl std::fmt::Display for VirtualTtyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VirtualTtyError::InvalidEscapeSequence(seq) => {
                write!(f, "Invalid escape sequence: {seq}")
            }
            VirtualTtyError::CursorOutOfBounds { row, col } => {
                write!(f, "Cursor position out of bounds: ({row}, {col})")
            }
            VirtualTtyError::InvalidParameter(param) => {
                write!(f, "Invalid parameter: {param}")
            }
        }
    }
}

impl std::error::Error for VirtualTtyError {}
