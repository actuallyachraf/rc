mod lex;
mod token;

fn main() {
    println!("Hello, world!");
    let tokens = lex::lex::lex(String::from(",=.{"));
    println!("{:?}", tokens);
    println!("{} ", token::to_string(token::Token::LBrace))
}
