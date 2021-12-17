use crate::token;
use crate::Lexer;
// Recursive Descent or Pratt Parsing are both candidates for the parser
// implementation.
// Design note :
// Our parser should consume tokens (instead of storing lex) this would
// alleviate some ownership issues.
//
// ```rust
// let tokens = lexer::new("int main(){ return 42; }").lex();
// let ast = parser::new(tokens.clone());
// ```
// Our parser when initated should keep a mutable iterator on Vec<Token>.
// next() -> returns the next token and move the iterator
// peek() -> returns the next token without moving the iterator
pub struct RDParser {
    tokens: Vec<token::Token>,
    curr: token::Token,
    next: token::Token,
}

impl RDParser {
    pub fn new(tokens: Vec<token::Token>) -> RDParser {
        let tokens = tokens;
        let mut iter = tokens.iter();
        let curr = iter.next().unwrap();
        let next = iter.next().unwrap();
        RDParser {
            tokens: tokens.clone(),
            curr: curr.clone(),
            next: next.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        let tokens = Lexer::new(String::from("{return 42;}")).lex();
        let parser = RDParser::new(tokens);
        assert_eq!(parser.curr, token::Token::LBrace);
    }
}
