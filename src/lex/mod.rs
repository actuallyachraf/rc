use crate::token;
extern crate regex;

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
    pub fn scan_identifier(&mut self) -> String {
        // scan an identifier
        // let ident = Regex::new(r"/[a-zA-Z_][a-zA-Z0-9]*/gm").unwrap();
        // let number_re = Regex::new(r"^([\d])+$").unwrap();
        // number_re.is_match(ident.as_str());
        let mut ident = String::new();
        while self.curr.is_ascii_alphanumeric() {
            ident.push(self.curr);
            self.advance();
        }
        ident
    }
    pub fn scan_number(&mut self) -> String {
        let mut ident = String::new();
        while self.peek().is_ascii_digit() {
            ident.push(self.curr);
            self.advance();
        }
        ident.push(self.curr);
        ident
    }
    pub fn skip_whitespace(&mut self) {
        while self.curr.is_ascii_whitespace() {
            self.advance()
        }
    }
    pub fn next(&mut self) -> token::Token {
        self.skip_whitespace();
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
            'a'..='z' | 'A'..='Z' => {
                let ident = self.scan_identifier();
                match ident.as_str() {
                    "return" => token::Token::Return,
                    "void" => token::Token::Void,
                    _ => {
                        let tag = format!("IDENTIFIER<{}>", ident);
                        token::Token::Ident(tag)
                    }
                }
            }
            '0'..='9' => {
                let number = self.scan_number();
                println!("NUMBER : {}", number);
                println!("CURRENT : {}", self.curr);
                token::Token::Int(number.parse::<i64>().unwrap())
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
            token::Token::LBrace,
            token::Token::Return,
            token::Token::Int(42),
            token::Token::SemiColon,
            token::Token::RBrace,
        ];
        let tokens = Lexer::new(String::from("{return 42;}")).lex();
        assert_eq!(tokens, expected);
    }
}
