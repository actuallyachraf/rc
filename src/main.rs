mod asm;
mod ast;
mod lex;
mod token;

use lex::Lexer;

fn main() {
    println!("Hello, world!");
    let tokens = Lexer::new(
        "int func(int a, int b){ return a + b ; } int main() { return func(5,3); }".to_string(),
    )
    .lex();
    println!("{:?}", tokens);
    println!("{}", asm::emit_ret())
}
