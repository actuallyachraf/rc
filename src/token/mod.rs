use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    EOF,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    SemiColon,
    Colon,
    Dot,
    Assign,
    Equal,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    BitAnd,
    BitOr,
    Xor,
    Shl,
    Shr,
    Bang,
    Complement,
    Question,
    Plus,
    Minus,
    Star,
    Div,
    Mod,
    Return,
    Void,
}

pub fn to_string(tok: Token) -> &'static str {
    match tok {
        Token::EOF => "\0",
        Token::LBrace => "{",
        Token::RBrace => "}",
        Token::LParen => "(",
        Token::RParen => ")",
        Token::Comma => ",",
        Token::SemiColon => ";",
        Token::Colon => ":",
        Token::Dot => ".",
        Token::Assign => "=",
        Token::Equal => "==",
        Token::Neq => "!=",
        Token::Gt => ">",
        Token::Lt => "<",
        Token::Gte => ">=",
        Token::Lte => "<=",
        Token::And => "&&",
        Token::Or => "||",
        Token::BitAnd => "&",
        Token::BitOr => "|",
        Token::Xor => "^",
        Token::Shl => "<<",
        Token::Shr => ">>",
        Token::Bang => "!",
        Token::Complement => "~",
        Token::Question => "?",
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Star => "*",
        Token::Div => "/",
        Token::Mod => "%",
        Token::Return => "RETURN",
        Token::Void => "VOID",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_string() {
        assert_eq!("}", to_string(Token::RBrace));
        assert_eq!("{", to_string(Token::LBrace));
        assert_eq!("(", to_string(Token::LParen));
        assert_eq!(")", to_string(Token::RParen));
        assert_eq!(",", to_string(Token::Comma));
        assert_eq!(";", to_string(Token::SemiColon));
        assert_eq!(":", to_string(Token::Colon));
        assert_eq!(".", to_string(Token::Dot));
        assert_eq!("=", to_string(Token::Assign));
        assert_eq!("==", to_string(Token::Equal));
    }
}
