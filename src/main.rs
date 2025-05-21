use std::env::args;
use std::fs;
use std::process::exit;

use cara::backend::Interpreter;
use cara::frontend::{Lexer, Parser};

fn main() {
    let options = getopts_macro::getopts_options! {
        -h  --help*         "Show help";
        -v  --version       "Show version";
    };
    let matched = match options.parse(args().skip(1)) {
        Ok(matched) => matched,
        Err(e) => {
            eprintln!("{e}");
            exit(2)
        },
    };
    if matched.opt_present("help") {
        print!("{}", options.usage("Usage: cara <source-file>"));
        exit(0)
    }
    if matched.opt_present("version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0)
    }

    for path in matched.free {
        let code = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read {}: {}", path, e);
            exit(1);
        });

        process_file(code);
    }
}

fn process_file(code: String) {
    let lexer = Lexer::new(code);
    let (ast, table) = Parser::new(lexer).parse_compile_unit();

    #[cfg(debug_assertions)]
    println!("{:#?}", ast);

    let mut interpreter = Interpreter::new(table);
    interpreter.set_printer(|args| print!("{}", args));

    match interpreter.visit(&ast) {
        #[cfg(debug_assertions)]
        Ok(value) => println!("{:?}", value),
        #[cfg(not(debug_assertions))]
        Ok(_) => (),
        Err(e) => eprintln!("Runtime error: {e}"),
    }
}
