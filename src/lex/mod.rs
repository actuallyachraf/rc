use crate::token;
extern crate regex;
use regex::Regex;

#[allow(dead_code)]
pub struct Lexer {
    input: String,
    chars: Vec<char>,
    curr: char,
    next: char,
    pos: usize,
    read_pos: usize,
    tokens: Vec<token::Token>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let chars: Vec<char> = input.chars().collect();
        let curr = '0';
        let next = '0';
        let read_pos = 0;
        let pos = 0;
        let tokens = Vec::<token::Token>::new();
        let mut l = Lexer {
            input,
            chars,
            curr,
            next,
            pos,
            read_pos,
            tokens,
        };
        l.advance();
        l
    }
    pub fn advance(&mut self) {
        if self.read_pos >= self.input.len() {
            // signal EOF
            self.curr = '\0';
            return;
        } else {
            self.curr = self.chars[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
    pub fn peek(&mut self) -> char {
        if self.read_pos >= self.input.len() {
            return 0 as char;
        }
        self.chars[self.read_pos]
    }
    pub fn scan(&mut self, ident: String) -> bool {
        // scan an identifier
        let number_re = Regex::new(r"^([\d])+$").unwrap();
        number_re.is_match(ident.as_str())
    }
    pub fn next(&mut self) -> token::Token {
        match self.curr {
            ',' => token::Token::Comma,
            ';' => token::Token::SemiColon,
            ':' => token::Token::Colon,
            '.' => token::Token::Dot,
            '(' => token::Token::LParen,
            ')' => token::Token::RParen,
            '}' => token::Token::RBrace,
            '{' => token::Token::LBrace,
            '^' => token::Token::Xor,
            '~' => token::Token::Complement,
            '?' => token::Token::Question,
            '+' => token::Token::Plus,
            '-' => token::Token::Minus,
            '*' => token::Token::Star,
            '/' => token::Token::Div,
            '%' => token::Token::Mod,
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    token::Token::And
                } else {
                    token::Token::BitAnd
                }
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    token::Token::Or
                } else {
                    token::Token::BitOr
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    token::Token::Gte
                } else if self.peek() == '>' {
                    self.advance();
                    token::Token::Shr
                } else {
                    token::Token::Gt
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    token::Token::Lte
                } else if self.peek() == '<' {
                    self.advance();
                    token::Token::Shl
                } else {
                    token::Token::Lt
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    token::Token::Neq
                } else {
                    token::Token::Bang
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    token::Token::Equal
                } else {
                    token::Token::Assign
                }
            }
            'a'..='z' | 'A'..='Z' => token::Token::Ident("IDENTIFIER<>".to_owned()),
            '0'..='9' => {
                let mut digit = Vec::new();
                while self.curr.is_ascii_digit() {
                    digit.push(self.curr);
                    self.advance()
                }
                let number: String = digit.into_iter().collect();
                token::Token::Ident(number)
            }
            _ => token::Token::EOF,
        }
    }

    pub fn lex(&mut self) -> Vec<token::Token> {
        while self.curr != '\0' {
            let token = self.next();
            self.tokens.push(token);
            self.advance();
        }
        self.tokens.clone()
    }
    pub fn tokens(&self) -> Vec<token::Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let expected = vec![
            token::Token::Comma,
            token::Token::Assign,
            token::Token::Dot,
            token::Token::LBrace,
            token::Token::Equal,
            token::Token::Ident(String::from("123")),
        ];
        let tokens = Lexer::new(String::from(",=.{==123")).lex();
        assert_eq!(tokens, expected);
    }
}
