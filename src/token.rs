use std::fmt;

pub enum TokenType {
    Integer,
    Plus,
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match *self {
            TokenType::Integer => "Integer",
            TokenType::Plus => "Plus",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", typ)
    }
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(value: String) -> Token {
        match value.as_str() {
            "+"   => Token{ kind: TokenType::Plus,    value: value },
            ""    => Token{ kind: TokenType::EOF,     value: value },
            "EOF" => Token{ kind: TokenType::EOF,     value: value },
            _     => Token{ kind: TokenType::Integer, value: value},
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TOKEN({}, {})", self.kind, self.value)
    }
}
