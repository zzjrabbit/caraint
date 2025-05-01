use std::env::args;
use std::fs;
use std::process::exit;

use cara::backend::Interpreter;
use cara::frontend::{Lexer, Parser};

fn main() {
    let path = args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cara <source-file>");
        exit(1);
    });

    let code = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", path, e);
        exit(1);
    });

    let lexer = Lexer::new(code);
    let ast = Parser::new(lexer).parse_compile_unit();

    #[cfg(debug_assertions)]
    println!("{:#?}", ast);

    let mut interpreter = Interpreter::new();
    interpreter.set_printer(|args| print!("{}", args));

    match interpreter.visit(&ast) {
        #[cfg(debug_assertions)]
        Ok(value) => println!("{:?}", value),
        #[cfg(not(debug_assertions))]
        Ok(_) => (),
        Err(e) => eprintln!("Runtime error: {e}"),
    }
}
