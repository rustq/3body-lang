use evaluator::env::Env;
use evaluator::object::*;
use evaluator::Evaluator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use lexer::Lexer;
use parser::Parser;

extern crate rand;
use evaluator::builtins::rand::distributions::Uniform;
use evaluator::builtins::rand::{thread_rng, Rng};

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin(1, monkey_len));
    builtins.insert(String::from("first"), Object::Builtin(1, monkey_first));
    builtins.insert(String::from("last"), Object::Builtin(1, monkey_last));
    builtins.insert(String::from("rest"), Object::Builtin(1, monkey_rest));
    builtins.insert(String::from("push"), Object::Builtin(2, monkey_push));
    builtins.insert(String::from("广播"), Object::Builtin(1, three_body_puts));
    builtins.insert(
        String::from("二向箔清理"),
        Object::Builtin(0, three_body_clear),
    );
    builtins.insert(String::from("毁灭"), Object::Builtin(0, three_body_exit));
    builtins.insert(String::from("冬眠"), Object::Builtin(1, three_body_sleep));
    builtins.insert(
        String::from("import"),
        Object::Builtin(1, three_body_import),
    );
    builtins.insert(
        String::from("random"),
        Object::Builtin(1, three_body_random),
    );
    builtins
}

fn monkey_len(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => Object::Int(s.len() as i64),
        Object::Array(o) => Object::Int(o.len() as i64),
        o => Object::Error(format!("argument to `len` not supported, got {}", o)),
    }
}

fn monkey_first(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if let Some(ao) = o.first() {
                ao.clone()
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `first` must be array. got {}", o)),
    }
}

fn monkey_last(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if let Some(ao) = o.last() {
                ao.clone()
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `last` must be array. got {}", o)),
    }
}

fn monkey_rest(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if !o.is_empty() {
                Object::Array(o[1..].to_vec())
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `rest` must be array. got {}", o)),
    }
}

fn monkey_push(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            let mut arr = o.clone();
            arr.push(args[1].clone());
            Object::Array(arr)
        }
        o => Object::Error(format!("argument to `push` must be array. got {}", o)),
    }
}

fn three_body_puts(args: Vec<Object>) -> Object {
    for arg in args {
        println!("{}", arg);
    }
    Object::Null
}

fn three_body_clear(_args: Vec<Object>) -> Object {
    std::process::Command::new("clear").status().unwrap();
    Object::Null
}

fn three_body_exit(_args: Vec<Object>) -> Object {
    std::process::exit(0);
}

fn three_body_sleep(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(o) => {
            let duration = std::time::Duration::from_millis((*o).try_into().unwrap());
            std::thread::sleep(duration);
            Object::Null
        }
        _ => Object::Null,
    }
}

fn three_body_import(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(o) => {
            let mut file = File::open(format!("{o}.3body")).expect("Unable to open the file");
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Unable to read the file");

            let mut parser = Parser::new(Lexer::new(&contents));
            let program = parser.parse();

            let env = Env::from(new_builtins());
            let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));

            let result = evaluator.eval(&program);

            match result {
                Some(obj) => return obj,
                _ => return Object::Null,
            };
        }
        _ => Object::Null,
    }
}

#[cfg(target_arch = "wasm32")]
fn three_body_random(args: Vec<Object>) -> Object {
    Object::Null
}

#[cfg(not(target_arch = "wasm32"))]
fn three_body_random(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(o) => {
            let mut rng = thread_rng();
            let n = rng.sample::<i64, _>(Uniform::new(0, *o));
            Object::Int(n)
        }
        _ => Object::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_len_string() {
        let args = vec![Object::String(String::from("hello"))];
        let expected = Object::Int(5);
        assert_eq!(monkey_len(args), expected);
    }

    #[test]
    fn test_monkey_len_array() {
        let args = vec![Object::Array(vec![
            Object::Int(1),
            Object::Int(2),
            Object::Int(3),
        ])];
        let expected = Object::Int(3);
        assert_eq!(monkey_len(args), expected);
    }

    #[test]
    fn test_monkey_len_error() {
        let args = vec![Object::Bool(true)];
        let expected = Object::Error(String::from("argument to `len` not supported, got true"));
        assert_eq!(monkey_len(args), expected);
    }

    #[test]
    fn test_monkey_first() {
        let args = vec![Object::Array(vec![
            Object::Int(1),
            Object::Int(2),
            Object::Int(3),
        ])];

        assert_eq!(monkey_first(args), Object::Int(1));

        let args = vec![Object::Array(vec![])];
        assert_eq!(monkey_first(args), Object::Null);

        let args = vec![Object::Int(1)];
        assert_eq!(
            monkey_first(args),
            Object::Error("argument to `first` must be array. got 1".to_string())
        );
    }
}
