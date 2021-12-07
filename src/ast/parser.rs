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