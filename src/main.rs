mod ast;

use ast::lexer::{Lexer, TokenKind};

fn main() {
    let source = "let x = 87; return x;";   
    let mut lexer = Lexer::new(source);

    println!("Source: {}", source);
    println!("--- Tokens ---");

    loop {
        let token = lexer.next_token().unwrap();
        println!("{:?} [{}]", token.kind, token.span.literal);

        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
