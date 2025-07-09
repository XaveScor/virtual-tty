use super::commands::AnsiCommand;

pub fn parse_escape_sequence(chars: &mut std::str::Chars) -> Option<AnsiCommand> {
    let mut params = String::new();
    let mut cmd = ' ';

    // Read until we find the command character
    for ch in chars {
        if ch.is_ascii_alphabetic() || ch == '~' {
            cmd = ch;
            break;
        }
        params.push(ch);
    }

    AnsiCommand::from_csi_command(cmd, &params)
}
