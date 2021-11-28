mod lex;
mod token;

fn main() {
    println!("Hello, world!");
    lex::lex::lex();
    lex::lex_mod();
    println!("{} ", token::to_string(token::Token::LBrace))
}
