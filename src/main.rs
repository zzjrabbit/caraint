use std::env::args;
use std::fs::File;
use std::io::Read;

use cara::backend::Interpreter;
use cara::frontend::{Lexer, Parser};

fn main() {
    let mut code = String::new();

    let path = args().nth(1).expect("Unable to get cara source file path!");
    File::open(path)
        .expect("Unable to find cara source file!")
        .read_to_string(&mut code)
        .expect("Unable to read cara source file!");

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
            panic!();
        },
    }
}
