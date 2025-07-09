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

impl AnsiCommand {
    pub fn from_csi_command(cmd: char, params: &str) -> Option<Self> {
        match cmd {
            'A' => {
                let n = params.parse::<usize>().unwrap_or(1);
                Some(AnsiCommand::CursorUp(n))
            }
            'B' => {
                let n = params.parse::<usize>().unwrap_or(1);
                Some(AnsiCommand::CursorDown(n))
            }
            'C' => {
                let n = params.parse::<usize>().unwrap_or(1);
                Some(AnsiCommand::CursorForward(n))
            }
            'D' => {
                let n = params.parse::<usize>().unwrap_or(1);
                Some(AnsiCommand::CursorBack(n))
            }
            'H' | 'f' => {
                let parts: Vec<&str> = params.split(';').collect();
                let row = parts
                    .first()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1)
                    .saturating_sub(1);
                let col = parts
                    .get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1)
                    .saturating_sub(1);
                Some(AnsiCommand::CursorPosition { row, col })
            }
            'J' => {
                let clear_mode = match params {
                    "2" => ClearMode::Entire,
                    "1" => ClearMode::ToBeginning,
                    "0" | "" => ClearMode::ToEnd,
                    _ => ClearMode::ToEnd,
                };
                Some(AnsiCommand::ClearScreen(clear_mode))
            }
            'K' => {
                let clear_mode = match params {
                    "2" => ClearMode::Entire,
                    "1" => ClearMode::ToBeginning,
                    "0" | "" => ClearMode::ToEnd,
                    _ => ClearMode::ToEnd,
                };
                Some(AnsiCommand::ClearLine(clear_mode))
            }
            'm' => Some(AnsiCommand::SetGraphicsRendition),
            _ => None,
        }
    }
}
