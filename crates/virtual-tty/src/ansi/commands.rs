#[derive(Debug, Clone, PartialEq)]
pub enum AnsiCommand {
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBack(usize),
    CursorPosition { row: usize, col: usize },
    ClearScreen(ClearMode),
    ClearLine(ClearMode),
    SetGraphicsRendition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClearMode {
    ToEnd,
    ToBeginning,
    Entire,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    Command(AnsiCommand),
    ControlChar(ControlChar),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlChar {
    LineFeed,
    CarriageReturn,
    Backspace,
    Tab,
    Bell,
    VerticalTab,
    FormFeed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidEscapeSequence(String),
    InvalidParameter(String),
    InvalidParameterCount {
        expected: usize,
        actual: usize,
    },
    InvalidParameterRange {
        param: String,
        min: usize,
        max: usize,
    },
    UnexpectedEndOfInput,
    InvalidCharacter(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidEscapeSequence(seq) => write!(f, "Invalid escape sequence: {}", seq),
            ParseError::InvalidParameter(param) => write!(f, "Invalid parameter: {}", param),
            ParseError::InvalidParameterCount { expected, actual } => {
                write!(
                    f,
                    "Invalid parameter count: expected {}, got {}",
                    expected, actual
                )
            }
            ParseError::InvalidParameterRange { param, min, max } => {
                write!(
                    f,
                    "Parameter '{}' out of range: must be between {} and {}",
                    param, min, max
                )
            }
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::InvalidCharacter(ch) => write!(f, "Invalid character: '{}'", ch),
        }
    }
}

impl std::error::Error for ParseError {}

impl AnsiCommand {
    pub fn from_csi_command(cmd: char, params: &[usize]) -> Result<Self, ParseError> {
        match cmd {
            'A' => {
                let n = params.first().copied().unwrap_or(1);
                let n = if n == 0 { 1 } else { n }; // Treat 0 as 1
                Ok(AnsiCommand::CursorUp(n))
            }
            'B' => {
                let n = params.first().copied().unwrap_or(1);
                let n = if n == 0 { 1 } else { n }; // Treat 0 as 1
                Ok(AnsiCommand::CursorDown(n))
            }
            'C' => {
                let n = params.first().copied().unwrap_or(1);
                let n = if n == 0 { 1 } else { n }; // Treat 0 as 1
                Ok(AnsiCommand::CursorForward(n))
            }
            'D' => {
                let n = params.first().copied().unwrap_or(1);
                let n = if n == 0 { 1 } else { n }; // Treat 0 as 1
                Ok(AnsiCommand::CursorBack(n))
            }
            'H' | 'f' => {
                let row = params.first().copied().unwrap_or(1).saturating_sub(1);
                let col = params.get(1).copied().unwrap_or(1).saturating_sub(1);
                Ok(AnsiCommand::CursorPosition { row, col })
            }
            'J' => {
                let param = params.first().copied().unwrap_or(0);
                let clear_mode = match param {
                    2 => ClearMode::Entire,
                    1 => ClearMode::ToBeginning,
                    0 => ClearMode::ToEnd,
                    _ => {
                        return Err(ParseError::InvalidParameterRange {
                            param: "clear_screen".to_string(),
                            min: 0,
                            max: 2,
                        })
                    }
                };
                Ok(AnsiCommand::ClearScreen(clear_mode))
            }
            'K' => {
                let param = params.first().copied().unwrap_or(0);
                let clear_mode = match param {
                    2 => ClearMode::Entire,
                    1 => ClearMode::ToBeginning,
                    0 => ClearMode::ToEnd,
                    _ => {
                        return Err(ParseError::InvalidParameterRange {
                            param: "clear_line".to_string(),
                            min: 0,
                            max: 2,
                        })
                    }
                };
                Ok(AnsiCommand::ClearLine(clear_mode))
            }
            'm' => Ok(AnsiCommand::SetGraphicsRendition),
            _ => Err(ParseError::InvalidEscapeSequence(format!(
                "Unknown CSI command: {}",
                cmd
            ))),
        }
    }

    pub fn validate(&self) -> Result<(), ParseError> {
        match self {
            AnsiCommand::CursorUp(n)
            | AnsiCommand::CursorDown(n)
            | AnsiCommand::CursorForward(n)
            | AnsiCommand::CursorBack(n) => {
                // Movement parameters should be valid (0 is already converted to 1 in parsing)
                if *n == 0 {
                    return Err(ParseError::InvalidParameterRange {
                        param: "cursor_movement".to_string(),
                        min: 1,
                        max: usize::MAX,
                    });
                }
            }
            AnsiCommand::CursorPosition { row, col } => {
                // Reasonable bounds for cursor position
                if *row > 65535 || *col > 65535 {
                    return Err(ParseError::InvalidParameterRange {
                        param: "cursor_position".to_string(),
                        min: 0,
                        max: 65535,
                    });
                }
            }
            _ => {
                // All other commands are valid by default
            }
        }
        Ok(())
    }
}
