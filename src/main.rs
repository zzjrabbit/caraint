use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use cara::backend::Interpreter;
use cara::frontend::{Lexer, Parser};

fn main() {
    let mut code = String::new();

    let path = args().nth(1).unwrap_or_else(|| {
        eprintln!("Unable to get cara source file path!");
        exit(1);
    });

    let mut file = File::open(&path).unwrap_or_else(|_| {
        eprintln!("Unable to find cara source file!");
        exit(1);
    });

    if file.read_to_string(&mut code).is_err() {
        eprintln!("Unable to read cara source file!");
        exit(1);
    }

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let (ast, strings) = parser.parse_compile_unit();

    #[cfg(debug_assertions)]
    println!("{:#?}", ast);

    cara::backend::set_printer(|args| print!("{}", args));

    let mut interpreter = Interpreter::new(strings);

    match interpreter.visit(&ast) {
        #[cfg(debug_assertions)]
        Ok(value) => println!("{:?}", value),
        #[cfg(not(debug_assertions))]
        Ok(_) => (),
        Err(e) => {
            eprintln!("on runtime error: {e}");
            eprintln!("variables:");
            for (i, name) in interpreter.string_table().iter().enumerate() {
                eprintln!(" {i}:\t{name}");
            }
            eprintln!("{e}");
        }
    }
}
