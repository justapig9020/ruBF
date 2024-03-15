use anyhow::{self, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Symbol {
    PlusOne,
    MinusOne,
    RightShift,
    LeftShift,
    RightBracket,
    LeftBracket,
    Output,
    Input,
}

impl TryFrom<char> for Symbol {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            '+' => Ok(Self::PlusOne),
            '-' => Ok(Self::MinusOne),
            '>' => Ok(Self::RightShift),
            '<' => Ok(Self::LeftShift),
            ']' => Ok(Self::RightBracket),
            '[' => Ok(Self::LeftBracket),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            _ => Err(anyhow::anyhow!("Invalid symbol")),
        }
    }
}

#[cfg(test)]
mod symbol {
    use super::*;
    #[test]
    fn test_try_from() {
        assert_eq!(Symbol::try_from('+').unwrap(), Symbol::PlusOne);
        assert_eq!(Symbol::try_from('-').unwrap(), Symbol::MinusOne);
        assert_eq!(Symbol::try_from('>').unwrap(), Symbol::RightShift);
        assert_eq!(Symbol::try_from('<').unwrap(), Symbol::LeftShift);
        assert_eq!(Symbol::try_from(']').unwrap(), Symbol::RightBracket);
        assert_eq!(Symbol::try_from('[').unwrap(), Symbol::LeftBracket);
        assert_eq!(Symbol::try_from('.').unwrap(), Symbol::Output);
        assert_eq!(Symbol::try_from(',').unwrap(), Symbol::Input);
        assert!(Symbol::try_from('a').is_err());
    }
}
