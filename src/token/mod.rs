use std::str;

pub enum Token {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    SemiColon,
    Colon,
    Dot,
}

pub fn to_string(tok: Token) -> &'static str {
    match tok {
        Token::LBrace => "{",
        Token::RBrace => "}",
        Token::LParen => "(",
        Token::RParen => ")",
        Token::Comma => ",",
        Token::SemiColon => ";",
        Token::Colon => ":",
        Token::Dot => ".",
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
    }
}
