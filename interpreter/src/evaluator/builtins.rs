use std::collections::HashMap;

extern crate rand;

use crate::evaluator::object::Object;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use llm::{load_progress_callback_stdout as load_callback, InferenceParameters, Model};
use llm_base::InferenceRequest;
use std::{convert::Infallible, io::Write, path::Path};
use spinoff;

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
    builtins.insert(
        String::from("智子工程"),
        Object::Builtin(1, three_body_sophon_engineering),
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

fn three_body_sophon_infer(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Hash(hash) => {
            let model_ptr = match hash.get(&Object::String("model".to_owned())).unwrap() {
                Object::NativeObject(model_ptr) => {
                    model_ptr.clone()
                },
                _ => panic!()
            };
            let character = hash.get(&Object::String("character".to_owned())).unwrap();
            let model = unsafe { & *model_ptr };

            let mut session = model.start_session(Default::default());
            let meessage = format!("{}", &args[1]);
            let prompt = &format!("
下面是描述一项任务的说明。需要适当地完成请求的响应。

### 角色设定:

{}

### 提问:

{}

### 回答:

", character, meessage);

            let sp = spinoff::Spinner::new(spinoff::spinners::Arc, "".to_string(), None);

            if let Err(llm::InferenceError::ContextFull) = session.feed_prompt::<Infallible>(
                model,
                &InferenceParameters {
                    ..Default::default()
                },
                prompt,
                &mut Default::default(),
                |t| {
                    Ok(())
                },
            ) {
                println!("Prompt exceeds context window length.")
            };
            sp.clear();

            let res = session.infer::<Infallible>(
                model,
                &mut thread_rng(),
                &InferenceRequest {
                    prompt: "",
                    ..Default::default()
                },
                // OutputRequest
                &mut Default::default(),
                |t| {
                    print!("{t}");
                    std::io::stdout().flush().unwrap();
    
                    Ok(())
                },
            );
    
            match res {
                Err(err) => println!("\n{err}"),
                _ => ()
            }
            Object::Null
        },
        _ => panic!()
    }
}



fn three_body_sophon_close(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Hash(hash) => {
            let model_ptr = match hash.get(&Object::String("model".to_owned())).unwrap() {
                Object::NativeObject(model_ptr) => {
                    model_ptr.clone()
                },
                _ => panic!()
            };
            // let model = unsafe { & *model_ptr };
            unsafe { Box::from_raw(model_ptr) };
            // std::mem::drop(model);
            Object::Null
        },
        _ => panic!()
    }
}


fn three_body_sophon_engineering(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Hash(o) => {
            let model_type = o[&Object::String("type".to_owned())].clone();
            let model_path = o[&Object::String("path".to_owned())].clone();
            let prompt = o[&Object::String("prompt".to_owned())].clone();

            let now = std::time::Instant::now();

            let model_type = {
                match model_type {
                    Object::String(model_type) => {
                        model_type
                    },
                    _ => {
                        panic!()
                    }
                }
            };

            let model_type = model_type.as_str();
            

            let model_path = {
                match model_path {
                    Object::String(path) => {
                        path
                    },
                    _ => {
                        panic!()
                    }
                }
            };

            let model_path = Path::new(model_path.as_str());

            let prompt = {
                match prompt {
                    Object::String(prompt) => {
                        prompt
                    },
                    _ => {
                        panic!()
                    }
                }
            };

            let character = prompt;

            let architecture = model_type.parse().unwrap_or_else(|e| panic!("{e}"));

            let model = llm::load_dynamic(architecture, model_path, Default::default(), load_callback)
                .unwrap_or_else(|err| {
                    panic!("Failed to load {model_type} model from {model_path:?}: {err}")
                });
            
            let model = Box::leak(model);

            println!(
                "智子工程初始化成功: 耗时 {} ms",
                now.elapsed().as_millis()
            );

            let model_ptr = &mut *model as *mut dyn Model;

            let mut session_hash = HashMap::new();
            session_hash.insert(Object::String("model".to_owned()), Object::NativeObject(model_ptr));
            session_hash.insert(Object::String("character".to_owned()), Object::String(character.to_string()));
            session_hash.insert(Object::String("infer".to_owned()), Object::Builtin(2, three_body_sophon_infer));
            session_hash.insert(Object::String("close".to_owned()), Object::Builtin(1, three_body_sophon_close));
            Object::Hash(session_hash)
        }
        _ => Object::Null,
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
