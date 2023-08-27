use evaluator::object::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
struct VariableStatus {
    editable: bool,
}

macro_rules! IS {
    () => (
        '是'
    )
}

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    pub store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Env>>>,
    variables_status: HashMap<String, VariableStatus>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
            variables_status: HashMap::new(),
        }
    }

    pub fn from(store: HashMap<String, Object>) -> Self {
        Env { store, outer: None, variables_status: HashMap::new() }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Env>>) -> Self {
        Env {
            store: HashMap::new(),
            outer: Some(outer),
            variables_status: HashMap::new()
        }
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        match self.store.get(&name) {
            Some(value) => Some(value.clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow_mut().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: &Object, editable: bool) -> Option<Object> {
        match self.variables_status.get(&name) {
            Some(variable_status) => {
                if !variable_status.editable {
                    let value = self.get(name.clone()).unwrap();
                    return Some(Object::Error(format!("{} {} {}!", &name, IS!(), value)))
                }
            },
            _ => {}
        }
        self.variables_status.insert(name.clone(), VariableStatus { editable });
        self.store.insert(name, value.clone());
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_new() {
        let env = Env::new();
        assert_eq!(env.store.len(), 0);
        assert_eq!(env.outer, None);
    }

    #[test]
    fn test_env_from() {
        let mut store = HashMap::new();
        store.insert("key".to_string(), Object::Int(1));
        let env = Env::from(store);
        assert_eq!(env.store.len(), 1);
        assert_eq!(env.outer, None);
    }

    #[test]
    fn test_env_new_with_outer() {
        let outer = Rc::new(RefCell::new(Env::new()));
        let env = Env::new_with_outer(outer.clone());
        assert_eq!(env.store.len(), 0);
        assert_eq!(env.outer, Some(outer));
    }

    #[test]
    fn test_env_get() {
        let mut env = Env::new();
        env.set("key".to_string(), &Object::Int(1), true);
        assert_eq!(env.get("key".to_string()), Some(Object::Int(1)));
    }

    #[test]
    fn test_env_set() {
        let mut env = Env::new();
        env.set("key".to_string(), &Object::Int(1), true);
        assert_eq!(env.store.get("key"), Some(&Object::Int(1)));
    }

    #[test]
    fn test_editable_false() {
        let mut env = Env::new();
        assert_eq!(env.set("key".to_string(), &Object::Int(1), false), None);
        assert_eq!(env.set("key".to_string(), &Object::Int(1), false), Some(Object::Error("key 是 1!".to_owned())));
    }
}
