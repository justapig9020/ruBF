#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    PlusOne,
    MinusOne,
    RightShift,
    LeftShift,
    RightBracket,
    LeftBracket,
    Output,
    Input,
    Slash,
    NewLine,
    EoF,
    WhiteSpace,
    Unknown(char),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::PlusOne,
            '-' => Self::MinusOne,
            '>' => Self::RightShift,
            '<' => Self::LeftShift,
            ']' => Self::RightBracket,
            '[' => Self::LeftBracket,
            '.' => Self::Output,
            ',' => Self::Input,
            '/' => Self::Slash,
            '\n' => Self::NewLine,
            ' ' | '\t' => Self::WhiteSpace,
            v => Self::Unknown(v),
        }
    }
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        char::from(value).into()
    }
}

#[cfg(test)]
mod token {
    use super::*;
    #[test]
    fn test_from_char() {
        assert_eq!(Token::from('+'), Token::PlusOne);
        assert_eq!(Token::from('-'), Token::MinusOne);
        assert_eq!(Token::from('>'), Token::RightShift);
        assert_eq!(Token::from('<'), Token::LeftShift);
        assert_eq!(Token::from(']'), Token::RightBracket);
        assert_eq!(Token::from('['), Token::LeftBracket);
        assert_eq!(Token::from('.'), Token::Output);
        assert_eq!(Token::from(','), Token::Input);
        assert_eq!(Token::from('/'), Token::Slash);
        assert_eq!(Token::from('\n'), Token::NewLine);
        assert_eq!(Token::from(' '), Token::WhiteSpace);
        assert_eq!(Token::from('\t'), Token::WhiteSpace);
    }
    #[test]
    fn test_from_u8() {
        assert_eq!(Token::from(b'+'), Token::PlusOne);
        assert_eq!(Token::from(b'-'), Token::MinusOne);
        assert_eq!(Token::from(b'>'), Token::RightShift);
        assert_eq!(Token::from(b'<'), Token::LeftShift);
        assert_eq!(Token::from(b']'), Token::RightBracket);
        assert_eq!(Token::from(b'['), Token::LeftBracket);
        assert_eq!(Token::from(b'.'), Token::Output);
        assert_eq!(Token::from(b','), Token::Input);
        assert_eq!(Token::from(b'/'), Token::Slash);
        assert_eq!(Token::from(b'\n'), Token::NewLine);
        assert_eq!(Token::from(b' '), Token::WhiteSpace);
        assert_eq!(Token::from(b'\t'), Token::WhiteSpace);
    }
}
