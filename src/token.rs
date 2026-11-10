mod kind;

use std::fmt;

use kind::Kind;

pub struct Token {
    kind: Kind,
    lexeme: String,
    literal: Option<String>,
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let kind = Kind::try_from(value)?;

        Ok(Token {
            kind,
            lexeme: value.to_string(),
            literal: None,
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = self.literal.as_deref().unwrap_or("null");

        let result = format!("{} {} {}", self.kind, self.lexeme, literal);
        write!(f, "{result}")
    }
}

impl Token {
    pub fn end_of_file() -> Self {
        Token {
            kind: Kind::EOF,
            lexeme: String::from(""),
            literal: None,
        }
    }
}
