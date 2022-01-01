#[derive(Debug,Clone, PartialEq)]
pub enum Token {
    Eof,
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
    Ident(String),
    Int(i64),
    Char(char),
    CInt,
    CChar,
}
pub fn to_string(tok: Token) -> String {
    match tok {
        Token::Eof => String::from("\0"),
        Token::LBrace => String::from("{"),
        Token::RBrace => String::from("}"),
        Token::LParen => String::from("("),
        Token::RParen => String::from(")"),
        Token::Comma => String::from(","),
        Token::SemiColon => String::from(";"),
        Token::Colon => String::from(":"),
        Token::Dot => String::from("."),
        Token::Assign => String::from("="),
        Token::Equal => String::from("=="),
        Token::Neq => String::from("!="),
        Token::Gt => String::from(">"),
        Token::Lt => String::from("<"),
        Token::Gte => String::from(">="),
        Token::Lte => String::from("<="),
        Token::And => String::from("&&"),
        Token::Or => String::from("||"),
        Token::BitAnd => String::from("&"),
        Token::BitOr => String::from("|"),
        Token::Xor => String::from("^"),
        Token::Shl => String::from("<<"),
        Token::Shr => String::from(">>"),
        Token::Bang => String::from("!"),
        Token::Complement => String::from("~"),
        Token::Question => String::from("?"),
        Token::Plus => String::from("+"),
        Token::Minus => String::from("-"),
        Token::Star => String::from("*"),
        Token::Div => String::from("/"),
        Token::Mod => String::from("%"),
        Token::Return => String::from("RETURN"),
        Token::Void => String::from("VOID"),
        Token::Ident(s) => format!("IDENT<{}>", s),
        Token::Int(x) => format!("INT<{}>", x),
        Token::Char(c) => format!("CHAR<{}>", c),
        Token::CInt => String::from("CINT"),
        Token::CChar => String::from("CCHAR"),
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
        assert_eq!("!=", to_string(Token::Neq));
        assert_eq!(">", to_string(Token::Gt));
        assert_eq!(">=", to_string(Token::Gte));
        assert_eq!("<", to_string(Token::Lt));
        assert_eq!("<=", to_string(Token::Lte));
        assert_eq!("&&", to_string(Token::And));
        assert_eq!("||", to_string(Token::Or));
        assert_eq!("&", to_string(Token::BitAnd));
        assert_eq!("|", to_string(Token::BitOr));
        assert_eq!("^", to_string(Token::Xor));
        assert_eq!("<<", to_string(Token::Shl));
        assert_eq!(">>", to_string(Token::Shr));
        assert_eq!("!", to_string(Token::Bang));
        assert_eq!("~", to_string(Token::Complement));
        assert_eq!("?", to_string(Token::Question));
        assert_eq!("+", to_string(Token::Plus));
        assert_eq!("-", to_string(Token::Minus));
        assert_eq!("*", to_string(Token::Star));
        assert_eq!("/", to_string(Token::Div));
        assert_eq!("%", to_string(Token::Mod));
        assert_eq!("RETURN", to_string(Token::Return));
        assert_eq!("VOID", to_string(Token::Void));
        assert_eq!("IDENT<a>", to_string(Token::Ident(String::from("a"))));
        assert_eq!("INT<5>", to_string(Token::Int(5)));
        assert_eq!("CHAR<b>", to_string(Token::Char('b')));
        assert_eq!("CINT", to_string(Token::CInt));
        assert_eq!("CCHAR", to_string(Token::CChar));
    }
}
