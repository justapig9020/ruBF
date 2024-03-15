#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    PlusOne,
    MinusOne,
    RightShift,
    LeftShift,
    RightBracket,
    LeftBracket,
    Output,
    Input,
    Unknown(char),
}

impl From<char> for Symbol {
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
            v => Self::Unknown(v),
        }
    }
}

impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        char::from(value).into()
    }
}

#[cfg(test)]
mod symbol {
    use super::*;
    #[test]
    fn test_from_char() {
        assert_eq!(Symbol::from('+'), Symbol::PlusOne);
        assert_eq!(Symbol::from('-'), Symbol::MinusOne);
        assert_eq!(Symbol::from('>'), Symbol::RightShift);
        assert_eq!(Symbol::from('<'), Symbol::LeftShift);
        assert_eq!(Symbol::from(']'), Symbol::RightBracket);
        assert_eq!(Symbol::from('['), Symbol::LeftBracket);
        assert_eq!(Symbol::from('.'), Symbol::Output);
        assert_eq!(Symbol::from(','), Symbol::Input);
    }
    #[test]
    fn test_from_u8() {
        assert_eq!(Symbol::from(b'+'), Symbol::PlusOne);
        assert_eq!(Symbol::from(b'-'), Symbol::MinusOne);
        assert_eq!(Symbol::from(b'>'), Symbol::RightShift);
        assert_eq!(Symbol::from(b'<'), Symbol::LeftShift);
        assert_eq!(Symbol::from(b']'), Symbol::RightBracket);
        assert_eq!(Symbol::from(b'['), Symbol::LeftBracket);
        assert_eq!(Symbol::from(b'.'), Symbol::Output);
        assert_eq!(Symbol::from(b','), Symbol::Input);
    }
}
