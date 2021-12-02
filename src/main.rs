mod asm;
mod ast;
mod lex;
mod token;

fn main() {
    println!("Hello, world!");
    let tokens = lex::Lexer::new(String::from(",=.{===")).lex();
    println!("{:?}", tokens);
    println!("{}", asm::emit_ret())
}
