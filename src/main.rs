mod lex;
mod token;

fn main() {
    println!("Hello, world!");
    let mut lexer = lex::Lexer::new(String::from(",=.{==="));
    lexer.lex();
    println!("{:?}", lexer.tokens());
}
