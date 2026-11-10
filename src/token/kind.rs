use std::fmt;

use anyhow::bail;

#[derive(Debug, Clone)]
pub enum Kind {
    LeftParen,
    RightParen,
    EOF,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Kind::LeftParen => "LEFT_PAREN",
            Kind::RightParen => "RIGHT_PAREN",
            Kind::EOF => "EOF",
        };

        write!(f, "{result}")
    }
}

impl TryFrom<char> for Kind {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '(' => Ok(Kind::LeftParen),
            ')' => Ok(Kind::RightParen),
            _ => bail!("Unexpected token found: {}", ch),
        }
    }
}
