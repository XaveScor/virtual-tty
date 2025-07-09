use super::commands::{AnsiCommand, ControlChar, ParseError, Token};

pub struct AnsiParser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> AnsiParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn parse(input: &'a str) -> Result<Vec<Token>, ParseError> {
        let mut parser = Self::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = parser.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, ParseError> {
        match self.chars.next() {
            None => Ok(None),
            Some('\x1b') => {
                if self.chars.peek() == Some(&'[') {
                    self.chars.next(); // consume '['
                    match self.parse_csi_sequence() {
                        Ok(command) => Ok(Some(Token::Command(command))),
                        Err(e) => Ok(Some(Token::Invalid(format!("CSI parse error: {:?}", e)))),
                    }
                } else {
                    Ok(Some(Token::Invalid(
                        "Incomplete escape sequence".to_string(),
                    )))
                }
            }
            Some('\n') => Ok(Some(Token::ControlChar(ControlChar::LineFeed))),
            Some('\r') => Ok(Some(Token::ControlChar(ControlChar::CarriageReturn))),
            Some('\x08') => Ok(Some(Token::ControlChar(ControlChar::Backspace))),
            Some('\t') => Ok(Some(Token::ControlChar(ControlChar::Tab))),
            Some('\x07') => Ok(Some(Token::ControlChar(ControlChar::Bell))),
            Some('\x0b') => Ok(Some(Token::ControlChar(ControlChar::VerticalTab))),
            Some('\x0c') => Ok(Some(Token::ControlChar(ControlChar::FormFeed))),
            Some(ch) => {
                let mut text = String::new();
                text.push(ch);

                // Collect consecutive text characters
                while let Some(&next_ch) = self.chars.peek() {
                    if next_ch == '\x1b'
                        || next_ch == '\n'
                        || next_ch == '\r'
                        || next_ch == '\x08'
                        || next_ch == '\t'
                        || next_ch == '\x07'
                        || next_ch == '\x0b'
                        || next_ch == '\x0c'
                    {
                        break;
                    }
                    text.push(self.chars.next().unwrap());
                }

                Ok(Some(Token::Text(text)))
            }
        }
    }

    fn parse_csi_sequence(&mut self) -> Result<AnsiCommand, ParseError> {
        let mut param_str = String::new();
        let mut command_char = None;

        // Read parameters and find command character
        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_alphabetic() || ch == '~' {
                command_char = Some(self.chars.next().unwrap());
                break;
            }
            param_str.push(self.chars.next().unwrap());
        }

        let cmd = command_char.ok_or(ParseError::UnexpectedEndOfInput)?;
        let params = self.parse_parameters(&param_str)?;

        AnsiCommand::from_csi_command(cmd, &params)
    }

    fn parse_parameters(&self, param_str: &str) -> Result<Vec<usize>, ParseError> {
        if param_str.is_empty() {
            return Ok(vec![]);
        }

        let mut params = Vec::new();
        for part in param_str.split(';') {
            if part.is_empty() {
                params.push(0);
            } else {
                let param = part.parse::<usize>().map_err(|_| {
                    ParseError::InvalidParameter(format!("Cannot parse parameter: {}", part))
                })?;
                params.push(param);
            }
        }

        Ok(params)
    }
}

// Legacy function for backward compatibility
pub fn parse_escape_sequence(chars: &mut std::str::Chars) -> Option<AnsiCommand> {
    let mut param_str = String::new();
    let mut command_char = None;

    // Read parameters and find command character
    for ch in chars {
        if ch.is_ascii_alphabetic() || ch == '~' {
            command_char = Some(ch);
            break;
        }
        param_str.push(ch);
    }

    let cmd = command_char?;

    // Parse parameters using the legacy string-based approach for compatibility
    let parsed_params = if param_str.is_empty() {
        vec![]
    } else {
        param_str
            .split(';')
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .collect()
    };

    AnsiCommand::from_csi_command(cmd, &parsed_params).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{AnsiCommand, ClearMode};

    #[test]
    fn test_parse_cursor_up() {
        let tokens = AnsiParser::parse("\x1b[1A").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command(AnsiCommand::CursorUp(n)) => assert_eq!(*n, 1),
            _ => panic!("Expected cursor up command"),
        }
    }

    #[test]
    fn test_parse_cursor_up_no_param() {
        let tokens = AnsiParser::parse("\x1b[A").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command(AnsiCommand::CursorUp(n)) => assert_eq!(*n, 1),
            _ => panic!("Expected cursor up command"),
        }
    }

    #[test]
    fn test_parse_cursor_position() {
        let tokens = AnsiParser::parse("\x1b[5;10H").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command(AnsiCommand::CursorPosition { row, col }) => {
                assert_eq!(*row, 4); // 1-based to 0-based
                assert_eq!(*col, 9); // 1-based to 0-based
            }
            _ => panic!("Expected cursor position command"),
        }
    }

    #[test]
    fn test_parse_clear_screen() {
        let tokens = AnsiParser::parse("\x1b[2J").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command(AnsiCommand::ClearScreen(ClearMode::Entire)) => {}
            _ => panic!("Expected clear screen command"),
        }
    }

    #[test]
    fn test_parse_mixed_content() {
        let tokens = AnsiParser::parse("Hello\x1b[2JWorld").unwrap();
        assert_eq!(tokens.len(), 3);

        match &tokens[0] {
            Token::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected text token"),
        }

        match &tokens[1] {
            Token::Command(AnsiCommand::ClearScreen(ClearMode::Entire)) => {}
            _ => panic!("Expected clear screen command"),
        }

        match &tokens[2] {
            Token::Text(text) => assert_eq!(text, "World"),
            _ => panic!("Expected text token"),
        }
    }

    #[test]
    fn test_parse_control_chars() {
        let tokens = AnsiParser::parse("Hello\nWorld\r").unwrap();
        assert_eq!(tokens.len(), 4);

        match &tokens[0] {
            Token::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected text token"),
        }

        match &tokens[1] {
            Token::ControlChar(ControlChar::LineFeed) => {}
            _ => panic!("Expected line feed"),
        }

        match &tokens[2] {
            Token::Text(text) => assert_eq!(text, "World"),
            _ => panic!("Expected text token"),
        }

        match &tokens[3] {
            Token::ControlChar(ControlChar::CarriageReturn) => {}
            _ => panic!("Expected carriage return"),
        }
    }

    #[test]
    fn test_parse_invalid_sequence() {
        let tokens = AnsiParser::parse("\x1b[999Z").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Invalid(_) => {}
            _ => panic!("Expected invalid token"),
        }
    }

    #[test]
    fn test_legacy_parser_compatibility() {
        let mut chars = "1A".chars();
        let command = parse_escape_sequence(&mut chars).unwrap();
        assert_eq!(command, AnsiCommand::CursorUp(1));
    }
}
