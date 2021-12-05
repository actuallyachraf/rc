use crate::lex;
use crate::token;
pub mod kind;

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

// We want to start by parsing expressions of the type
// int a = 5; which could be translated to the ast form as
// Kind Identifier<a> = Int(5)
// Where Kind represents the type.

trait Node {
    fn token(&mut self) -> token::Token;
}
#[derive(Debug, Clone, PartialEq)]
struct Ident {
    tok: token::Token,
    val: String,
}
#[derive(Debug, Clone, PartialEq)]
struct Statement {
    ctype: kind::Kind,
    name: Ident,
    value: i64,
}

pub struct Prog {
    statements: Vec<Statement>,
}

pub struct Parser {
    lexer: lex::Lexer,
    curr: token::Token,
    next: token::Token,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = lex::Lexer::new(input);
        let tokens = lexer.lex();
        let curr = tokens[0].clone();
        let next = tokens[1].clone();

        Parser { lexer, curr, next }
    }
    pub fn parse(&mut self) -> Prog {
        Prog {
            statements: Vec::new(),
        }
    }
    pub fn parse_declaration(&mut self) -> Statement {
        let mut ctype: kind::Kind = kind::Kind::Void;
        let mut name: Ident = Ident {
            tok: token::Token::Ident("a".to_string()),
            val: "a".to_string(),
        };
        let mut value: i64 = 0;
        let tokens = self.lexer.tokens();
        let mut iter = tokens.iter();
        self.curr = iter.next().unwrap().clone();
        self.next = iter.next().unwrap().clone();

        match &self.curr {
            token::Token::CInt => ctype = kind::Kind::Int,
            token::Token::CChar => ctype = kind::Kind::Char,
            _ => ctype= kind::Kind::Void,
        }
        match &self.next {
            token::Token::Ident(s) => {
                name = Ident {
                    tok: token::Token::Ident(s.to_string()),
                    val: s.clone(),
                }
            }
            _ => println!("ERROR"),
        }
        self.curr = iter.next().unwrap().clone();
        self.next = iter.next().unwrap().clone();

        match &self.curr {
            token::Token::Assign => match &self.next {
                token::Token::Int(k) => {
                    value = *k;
                }
                _ => println!("ERROR"),
            },
            _ => println!("ERROR"),
        }
        Statement { ctype, name, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_parse_declaration() {
        let expected = Statement {
            ctype: kind::Kind::Int,
            name: Ident {
                tok: token::Token::Ident("year".to_string()),
                val: "year".to_string(),
            },
            value: 2019,
        };
        let input = "int year=2019;".to_string();
        let mut parser = Parser::new(input);
        let decl = parser.parse_declaration();
        println!("{:?}", decl);
        assert_eq!(expected, decl);
    }
}
