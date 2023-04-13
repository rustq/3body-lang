#![allow(clippy::derive_hash_xor_eq)]

use ast::*;
use evaluator::env::*;
use lexer::unescape::escape_str;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type BuiltinFunc = fn(Vec<Object>) -> Object;

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Int(i64),
    String(String),
    Bool(bool),
    Array(Vec<Object>),
    Hash(HashMap<Object, Object>),
    Func(Vec<Ident>, BlockStmt, Rc<RefCell<Env>>),
    Builtin(i32, BuiltinFunc),
    Null,
    BreakStatement,
    ContinueStatement,
    ReturnValue(Box<Object>),
    Error(String),
}

/// This is actually repr
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(ref value) => write!(f, "{}", value),
            Object::String(ref value) => write!(f, "{}", escape_str(value)),
            Object::Bool(ref value) => write!(f, "{}", value),
            Object::Array(ref objects) => {
                let mut result = String::new();
                for (i, obj) in objects.iter().enumerate() {
                    if i < 1 {
                        result.push_str(&format!("{}", obj));
                    } else {
                        result.push_str(&format!(", {}", obj));
                    }
                }
                write!(f, "[{}]", result)
            }
            Object::Hash(ref hash) => {
                let mut result = String::new();
                for (i, (k, v)) in hash.iter().enumerate() {
                    if i < 1 {
                        result.push_str(&format!("{}: {}", k, v));
                    } else {
                        result.push_str(&format!(", {}: {}", k, v));
                    }
                }
                write!(f, "{{{}}}", result)
            }
            Object::Func(ref params, _, _) => {
                let mut result = String::new();
                for (i, Ident(ref s)) in params.iter().enumerate() {
                    if i < 1 {
                        result.push_str(&s.to_string());
                    } else {
                        result.push_str(&format!(", {}", s));
                    }
                }
                write!(f, "fn({}) {{ ... }}", result)
            }
            Object::Builtin(_, _) => write!(f, "[builtin function]"),
            Object::Null => write!(f, "null"),
            Object::BreakStatement => write!(f, "BreakStatement"),
            Object::ContinueStatement => write!(f, "ContinueStatement"),
            Object::ReturnValue(ref value) => write!(f, "ReturnValue({})", value),
            Object::Error(ref value) => write!(f, "Error({})", value),
        }
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Object::Int(ref i) => i.hash(state),
            Object::Bool(ref b) => b.hash(state),
            Object::String(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_int() {
        let obj = Object::Int(42);
        assert_eq!(obj.to_string(), "42");
    }

    #[test]
    fn test_object_string() {
        let obj = Object::String("hello".to_string());
        assert_eq!(obj.to_string(), "\"hello\"");
    }

    #[test]
    fn test_object_bool() {
        let obj = Object::Bool(true);
        assert_eq!(obj.to_string(), "true");
    }

    #[test]
    fn test_object_array() {
        let obj = Object::Array(vec![Object::Int(1), Object::Int(2), Object::Int(3)]);
        assert_eq!(obj.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_object_hash() {
        let mut hash = HashMap::new();
        hash.insert(Object::String("foo".to_string()), Object::Int(42));
        hash.insert(Object::String("bar".to_string()), Object::Int(1337));
        let obj = Object::Hash(hash);
        assert_eq!(obj.to_string(), "{\"foo\": 42, \"bar\": 1337}");
    }

    #[test]
    fn test_object_func() {
        let obj = Object::Func(
            vec![Ident("x".to_string()), Ident("y".to_string())],
            vec![],
            Rc::new(RefCell::new(Env::new())),
        );
        assert_eq!(format!("{}", obj), "fn(x, y) { ... }");
    }

    #[test]
    fn test_object_builtin() {
        let obj = Object::Builtin(0, |args| Object::Int(args.len() as i64));
        assert_eq!(obj.to_string(), "[builtin function]");
    }

    #[test]
    fn test_object_null() {
        let obj = Object::Null;
        assert_eq!(obj.to_string(), "null");
    }

    #[test]
    fn test_object_break() {
        let obj = Object::BreakStatement;
        assert_eq!(obj.to_string(), "BreakStatement");
    }

    #[test]
    fn test_object_continue() {
        let obj = Object::ContinueStatement;
        assert_eq!(obj.to_string(), "ContinueStatement");
    }

    #[test]
    fn test_object_return_value() {
        let obj = Object::ReturnValue(Box::new(Object::Int(42)));
        assert_eq!(obj.to_string(), "ReturnValue(42)");
    }

    #[test]
    fn test_object_error() {
        let obj = Object::Error("something went wrong".to_string());
        assert_eq!(obj.to_string(), "Error(something went wrong)");
    }
}
