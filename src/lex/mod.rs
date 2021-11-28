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
    pub fn new(s: String) -> Lexer {
        let chars: Vec<char> = s.chars().collect();
        let curr = '0';
        let next = '0';
        let read_pos = 0;
        let pos = 0;
        let tokens = Vec::<token::Token>::new();
        let mut l = Lexer {
            input: s,
            chars: chars,
            curr: curr,
            next: next,
            pos: pos,
            read_pos: read_pos,
            tokens: tokens,
        };
        l.read_char();
        l
    }
    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            // signal EOF
            self.curr = '0';
        } else {
            self.curr = self.chars[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos = self.read_pos + 1;
    }
    pub fn peek_char(&mut self) -> char {
        if self.read_pos >= self.input.len() {
            return 0 as char;
        }
        return self.chars[self.read_pos];
    }
    pub fn next_token(&mut self) -> token::Token {
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
                if self.peek_char() == '=' {
                    self.read_char();
                    return token::Token::Equal;
                } else {
                    return token::Token::Assign;
                }
            }
            _ => token::Token::EOF,
        }
    }

    pub fn lex(&mut self) {
        for c in self.chars.clone() {
            println!("Char : {}", c);
            println!("Current : {}", self.curr);
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
                    if self.peek_char() == '=' {
                        self.tokens.push(token::Token::Equal);
                        self.read_char();
                    } else {
                        self.tokens.push(token::Token::Assign)
                    }
                }
                _ => return,
            }
            self.read_char()
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
        assert_eq!(expected == lexer.tokens(), true);
    }
}
