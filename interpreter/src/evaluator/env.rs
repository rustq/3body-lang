use crate::evaluator::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
struct VariableStatus {
    constant: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    pub identifiers: HashMap<String, Object>,
    variables_status: HashMap<String, VariableStatus>,
    outer: Option<Rc<RefCell<Env>>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Info {
    ConstantForbidden,
    NoIdentifier,
    Succeed,
}

impl Env {
    pub fn new() -> Self {
        Env {
            identifiers: HashMap::new(),
            variables_status: HashMap::new(),
            outer: None,
        }
    }

    pub fn from(builtins: HashMap<String, Object>) -> Self {
        Env {
            identifiers: builtins,
            variables_status: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Env>>) -> Self {
        Env {
            identifiers: HashMap::new(),
            variables_status: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.identifiers.insert(name, value.clone());
    }

    pub fn update(&mut self, name: String, value: Object) -> Info {
        match self.identifiers.contains_key(&name) {
            true => {
                if self.is_constant(name.clone()) {
                    return Info::ConstantForbidden;
                }
                self.identifiers.insert(name.clone(), value.clone());
                return Info::Succeed;
            },
            false => {
                match self.outer {
                    Some(ref outer) => outer.borrow_mut().update(name, value),
                    None => Info::NoIdentifier,
                }
            }
        }
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        match self.identifiers.get(&name) {
            Some(value) => Some(value.clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow_mut().get(name),
                None => None,
            },
        }
    }
}

impl Env {
    pub fn constant(&mut self, name: String) {
        match self.variables_status.get_mut(&name) {
            Some(variable_status) => {
                variable_status.constant = true;
            }
            None => {
                self.variables_status
                    .insert(name, VariableStatus { constant: true });
            }
        }
    }

    pub fn is_constant(&mut self, name: String) -> bool {
        match self.variables_status.get(&name) {
            Some(variable_status) => variable_status.constant == true,
            None => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;
    use crate::evaluator::object::*;

    #[test]
    fn test_env_new() {
        let env = Env::new();
        assert_eq!(env.identifiers.len(), 0);
        assert_eq!(env.outer, None);
    }

    #[test]
    fn test_env_from() {
        let mut store = HashMap::new();
        store.insert("key".to_string(), Object::Int(1));
        let env = Env::from(store);
        assert_eq!(env.identifiers.len(), 1);
        assert_eq!(env.outer, None);
    }

    #[test]
    fn test_env_new_with_outer() {
        let outer = Rc::new(RefCell::new(Env::new()));
        let env = Env::new_with_outer(outer.clone());
        assert_eq!(env.identifiers.len(), 0);
        assert_eq!(env.outer, Some(outer));
    }

    #[test]
    fn test_env_get() {
        let mut env = Env::new();
        env.set("key".to_string(), Object::Int(1));
        assert_eq!(env.get("key".to_string()), Some(Object::Int(1)));
    }

    #[test]
    fn test_env_set() {
        let mut env = Env::new();
        env.set("key".to_string(), Object::Int(1));
        assert_eq!(env.identifiers.get("key"), Some(&Object::Int(1)));
    }

    // self cases

    #[test]
    fn test_update_succeed() {
        let mut env = Env::new();
        env.set("key".to_string(), Object::Int(1));
        assert_eq!(env.update("key".to_string(), Object::Int(2)), Info::Succeed);
    }

    #[test]
    fn test_update_forbidden() {
        let mut env = Env::new();
        env.set("key".to_string(), Object::Int(1));
        env.constant("key".to_string());
        assert_eq!(env.update("key".to_string(), Object::Int(2)), Info::ConstantForbidden);
    }

    #[test]
    fn test_update_no_identifier() {
        let mut env = Env::new();
        assert_eq!(env.update("key".to_string(), Object::Int(2)), Info::NoIdentifier);
    }

    #[test]
    fn test_update_out_identifier() {
        let global = Rc::new(RefCell::new(Env::new()));
        let outer = Rc::new(RefCell::new(Env::new_with_outer(global.clone())));
        let mut env = Env::new_with_outer(outer.clone());
        outer.as_ref().borrow_mut().set("key".to_string(), Object::Int(2));
        assert_eq!(outer.as_ref().borrow_mut().update("key".to_string(), Object::Int(2)), Info::Succeed);
        assert_eq!(env.update("key".to_string(), Object::Int(2)), Info::Succeed);
        assert_eq!(global.as_ref().borrow_mut().update("key".to_string(), Object::Int(2)), Info::NoIdentifier);
    }
}
