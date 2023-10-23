#[cfg(feature = "repl")]
extern crate rustyline;

use three_body_e2021::evaluator::builtins::new_builtins;
use three_body_e2021::evaluator::env;
use three_body_e2021::evaluator::Evaluator;
use three_body_e2021::evaluator::object;
use three_body_e2021::lexer::Lexer;
use three_body_e2021::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut rl = rustyline::DefaultEditor::new().expect("should exist");

    let mut evaluator = Evaluator {
        env: Rc::new(RefCell::new(env::Env::from(new_builtins()))),
    };

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if (args[1] == "-v") {
            println!("0.3.0");
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
