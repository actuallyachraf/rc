use crate::token;

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
        l.next_char();
        l
    }
    pub fn next_char(&mut self) {
        if self.read_pos >= self.input.len() {
            // signal EOF
            self.curr = '0';
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
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    token::Token::Equal
                } else {
                    token::Token::Assign
                }
            }
            _ => token::Token::EOF,
        }
    }

    pub fn lex(&mut self) {
        for _ in 0..self.chars.len() {
            match self.curr {
                ',' => self.tokens.push(token::Token::Comma),
                ';' => self.tokens.push(token::Token::SemiColon),
                ':' => self.tokens.push(token::Token::Colon),
                '.' => self.tokens.push(token::Token::Dot),
                '(' => self.tokens.push(token::Token::LParen),
                ')' => self.tokens.push(token::Token::RParen),
                '}' => self.tokens.push(token::Token::RBrace),
                '{' => self.tokens.push(token::Token::LBrace),
                '=' => {
                    if self.peek() == '=' {
                        self.tokens.push(token::Token::Equal);
                        self.next_char();
                    } else {
                        self.tokens.push(token::Token::Assign)
                    }
                }
                _ => return,
            }
            self.next_char()
        }
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
        ];
        let mut lexer = Lexer::new(String::from(",=.{=="));
        lexer.lex();
        assert!(expected == lexer.tokens());
    }
}
