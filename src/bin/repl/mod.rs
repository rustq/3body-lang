#[cfg(feature = "repl")]
extern crate rustyline;

pub mod helper;

use three_body_interpreter::evaluator::builtins::new_builtins;
use three_body_interpreter::evaluator::env;
use three_body_interpreter::evaluator::Evaluator;
use three_body_interpreter::evaluator::object;
use three_body_interpreter::lexer::Lexer;
use three_body_interpreter::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    let mut rl = rustyline::Editor::new().expect("should exist");

    rl.set_helper(Some(helper::Helper::new()));

    let mut evaluator = Evaluator {
        env: Rc::new(RefCell::new(env::Env::from(new_builtins()))),
    };

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-V" => {
                println!(env!("CARGO_PKG_VERSION"));
            }
            "-c" => {
                let input = args[2].to_owned();
                let mut lexer = Lexer::new(&input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse();
                let errors = parser.get_errors();

                if errors.len() > 0 {
                    for err in errors {
                        println!("{:?}", err);
                    }
                    return;
                }

                if let Some(evaluated) = evaluator.eval(&program) {
                    match evaluated {
                        object::Object::Null => {},
                        _ => println!("{}\n", evaluated),
                    }
                }
            }
            _ => {
                println!("usage: 3body [option] ... [arg] ...

Options and arguments:

-V     : print the 3body-lang version number and exit 
-h     : print this help message and exit 
-c cmd : program passed in as string (terminates option list)
-      : program in repl (default)
")
            }
        }
        return;
    }

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                let mut lexer = Lexer::new(&line);
                let mut parser = Parser::new(lexer);
                let program = parser.parse();
                let errors = parser.get_errors();

                if errors.len() > 0 {
                    for err in errors {
                        println!("{:?}", err);
                    }
                    continue;
                }

                if let Some(evaluated) = evaluator.eval(&program) {
                    match evaluated {
                        object::Object::Null => {},
                        _ => println!("{}\n", evaluated),
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("\n文明的种子仍在，它将重新启动，再次开始在三体世界中命运莫测的进化，欢迎您再次登录。");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
