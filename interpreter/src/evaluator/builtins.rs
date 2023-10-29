use std::collections::HashMap;

extern crate rand;

use crate::evaluator::object::Object;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

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
        String::from("random"),
        Object::Builtin(1, three_body_random),
    );
    builtins.insert(
        String::from("没关系的都一样"),
        Object::Builtin(2, three_body_deep_equal),
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

fn three_body_deep_equal(args: Vec<Object>) -> Object {
    if format!("{}", &args[0]) == format!("{}", &args[1]) {
        Object::Bool(true)
    } else {
        Object::Bool(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::evaluator::env::Env;
    use std::cell::RefCell;
    use std::rc::Rc;

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

    #[test]
    fn test_monkey_last() {
        let arr = vec![Object::Int(1), Object::Int(2), Object::Int(3)];
        let args = vec![Object::Array(arr)];
        assert_eq!(monkey_last(args), Object::Int(3));
    }

    #[test]
    fn test_monkey_rest() {
        let tests = vec![
            (
                vec![Object::Array(vec![Object::Int(1), Object::Int(2)])],
                Object::Array(vec![Object::Int(2)]),
            ),
            (
                vec![Object::Array(vec![Object::Int(1)])],
                Object::Array(vec![]),
            ),
            (vec![Object::Array(vec![])], Object::Null),
            (
                vec![Object::Int(1)],
                Object::Error("argument to `rest` must be array. got 1".to_string()),
            ),
        ];

        for (input, expected) in tests {
            let got = monkey_rest(input);
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn test_monkey_push() {
        let arr = vec![Object::Int(1), Object::Int(2)];
        let args = vec![Object::Array(arr), Object::Int(3)];
        let expected = Object::Array(vec![Object::Int(1), Object::Int(2), Object::Int(3)]);
        assert_eq!(monkey_push(args), expected);
    }

    #[test]
    fn test_three_body_deep_equal() {
        let tests = vec![
            (vec![Object::Int(1), Object::Int(1)], Object::Bool(true)),
            (vec![Object::Int(2), Object::Int(1)], Object::Bool(false)),
            (
                vec![
                    Object::String(String::from("hello")),
                    Object::String(String::from("hello")),
                ],
                Object::Bool(true),
            ),
            (
                vec![
                    Object::String(String::from("hello")),
                    Object::String(String::from("bye")),
                ],
                Object::Bool(false),
            ),
            (
                vec![Object::Bool(true), Object::Bool(true)],
                Object::Bool(true),
            ),
            (
                vec![Object::Bool(true), Object::Bool(false)],
                Object::Bool(false),
            ),
            (
                vec![
                    Object::Array(vec![
                        Object::Int(1),
                        Object::Int(2),
                        Object::Int(3),
                        Object::String(String::from("hello")),
                        Object::Bool(true),
                        Object::Array(vec![Object::Int(1)]),
                    ]),
                    Object::Array(vec![
                        Object::Int(1),
                        Object::Int(2),
                        Object::Int(3),
                        Object::String(String::from("hello")),
                        Object::Bool(true),
                        Object::Array(vec![Object::Int(1)]),
                    ]),
                ],
                Object::Bool(true),
            ),
            (
                vec![
                    Object::Array(vec![
                        Object::Int(1),
                        Object::Int(2),
                        Object::Int(3),
                        Object::String(String::from("hello")),
                        Object::Bool(true),
                    ]),
                    Object::Array(vec![]),
                ],
                Object::Bool(false),
            ),
            (
                vec![
                    {
                        let mut hash = HashMap::new();
                        hash.insert(Object::String("a".to_string()), Object::Int(1));
                        Object::Hash(hash)
                    },
                    {
                        let mut hash = HashMap::new();
                        hash.insert(Object::String("a".to_string()), Object::Int(1));
                        Object::Hash(hash)
                    },
                ],
                Object::Bool(true),
            ),
            (
                vec![
                    {
                        let mut hash = HashMap::new();
                        hash.insert(Object::String("a".to_string()), Object::Int(1));
                        Object::Hash(hash)
                    },
                    {
                        let mut hash = HashMap::new();
                        hash.insert(Object::String("b".to_string()), Object::Int(2));
                        Object::Hash(hash)
                    },
                ],
                Object::Bool(false),
            ),
            (
                vec![
                    Object::Builtin(1, monkey_first),
                    Object::Builtin(1, monkey_first),
                ],
                Object::Bool(true),
            ),
            (
                vec![
                    Object::Function(
                        vec![ast::Ident(String::from("x"))],
                        vec![],
                        Rc::new(RefCell::new(Env::new())),
                    ),
                    Object::Function(
                        vec![ast::Ident(String::from("x"))],
                        vec![],
                        Rc::new(RefCell::new(Env::new())),
                    ),
                ],
                Object::Bool(true),
            ),
            (
                vec![
                    Object::Function(
                        vec![ast::Ident(String::from("x"))],
                        vec![],
                        Rc::new(RefCell::new(Env::new())),
                    ),
                    Object::Function(
                        vec![ast::Ident(String::from("y"))],
                        vec![],
                        Rc::new(RefCell::new(Env::new())),
                    ),
                ],
                Object::Bool(false),
            ),
            (
                vec![
                    Object::Builtin(1, monkey_first),
                    Object::Function(
                        vec![ast::Ident(String::from("x"))],
                        vec![],
                        Rc::new(RefCell::new(Env::new())),
                    ),
                ],
                Object::Bool(false),
            ),
            (
                vec![Object::Int(1), Object::String(String::from("2"))],
                Object::Bool(false),
            ),
        ];

        for (input, expected) in tests {
            let got = three_body_deep_equal(input);
            assert_eq!(got, expected);
        }
    }
}
