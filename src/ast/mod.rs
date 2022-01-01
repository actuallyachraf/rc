use crate::token;
pub mod kind;
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
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Equality,
    Unary,
}
// We want to start by parsing expressions of the type
// int a = 5; which could be translated to the ast form as
// Kind Identifier<a> = Int(5)
// Where Kind represents the type.

trait Node {
    fn token(&mut self) -> token::Token;
}
#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    tok: token::Token,
    val: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    ctype: kind::Kind,
    name: Ident,
    value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    init:Expression,
    cond:Expression,
    update:Expression,
    body:Vec<Statement>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    tok:token::Token,
    val:i64,
}

pub struct Prog {
    statements: Vec<Statement>,
}

pub struct Parser {
    tokens:Vec<token::Token>,
    curr: token::Token,
    next: token::Token,
    pos:usize,
}

impl Parser {
    pub fn new(input: Vec<token::Token>) -> Parser {
        let curr = input[0].clone();
        let next = input[1].clone();
        Parser { tokens:input,curr, next , pos : 1}
    }
    fn advance(&mut self) {
        if self.pos+1 >= self.tokens.len() {
            return
        }
        self.curr = self.tokens[self.pos].clone();
        self.next = self.tokens[self.pos+1].clone();
        self.pos+=1;
    }
    pub fn parse(&mut self) -> Prog {
        Prog {
            statements: Vec::new(),
        }
    }
    pub fn parse_statement(&mut self) -> ReturnStatement {
        match &self.curr {
            token::Token::Return => self.parse_ret_statement(),
            _ => ReturnStatement{
                tok:token::Token::Return,
                val:0,
            },
        }
    }
    pub fn parse_ret_statement(&mut self) -> ReturnStatement {
        let mut statement = ReturnStatement {
            tok:self.curr.clone(),
            val:0,
        };
        match &self.next {
            token::Token::Int(k) => statement.val = *k,
            _ => statement.val = 0,
        }
        while self.next != token::Token::SemiColon {
            self.advance()
        }
        statement
    }
    pub fn parse_declaration(&mut self) -> Statement {
        // TODO: Handle Function Declarations
        let mut ctype: kind::Kind = kind::Kind::Void;
        let mut name: Ident = Ident {
            tok: token::Token::Ident("a".to_string()),
            val: "a".to_string(),
        };
        let mut value: i64 = 0;
        let tokens = &self.tokens;
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
    pub fn parse_expression(&mut self) -> Expression {
        // TODO: Parse expressions using RD or Pratt
        match &self.curr {
            token::Token::Equal => {
                Expression::Equality
            }
            _ => {
                Expression::Unary
            }
        }
    }
    // TODO: Parse for statements
    /*
    fn parse_for_statement(&mut self) -> ForStatement {
        let tokens = self.lexer.tokens();
        let mut iter = tokens.iter();
        self.curr = iter.next().unwrap().clone();
        self.next = iter.next().unwrap().clone();
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex;
    fn prepare(input:String) -> Vec<token::Token> {
        let mut lexer = lex::Lexer::new(input);
        lexer.lex()
    }
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
        let input = prepare("int year=2019;".to_string());
        let mut parser = Parser::new(input);
        let decl = parser.parse_declaration();
        println!("{:?}", decl);
        assert_eq!(expected, decl);
    }
    #[test]
    pub fn test_parse_declaration_ident() {
        let expected = Statement {
            ctype: kind::Kind::Int,
            name: Ident {
                tok: token::Token::Ident("id".to_string()),
                val: "id".to_string(),
            },
            value: 12345,
        };
        let input = prepare("int id = 12345;".to_string());
        let mut parser = Parser::new(input);
        let decl = parser.parse_declaration();
        println!("{:?}", decl);
        assert_eq!(expected, decl);
    }
    #[test]
    pub fn test_parse_return() {
        let expected = ReturnStatement {
            tok:token::Token::Return,
            val:42,
        };
        let input = prepare("return 42;".to_string());
        let mut parser = Parser::new(input);
        let decl = parser.parse_statement();
        println!("{:?}", decl);
        assert_eq!(expected, decl);
    }
}
