use evaluator::object::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    pub store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn from(store: HashMap<String, Object>) -> Self {
        Env { store, outer: None }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Env>>) -> Self {
        Env {
            store: HashMap::new(),
            outer: Some(outer),
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

    pub fn set(&mut self, name: String, value: &Object) {
        self.store.insert(name, value.clone());
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
        env.set("key".to_string(), &Object::Int(1));
        assert_eq!(env.get("key".to_string()), Some(Object::Int(1)));
    }

    #[test]
    fn test_env_set() {
        let mut env = Env::new();
        env.set("key".to_string(), &Object::Int(1));
        assert_eq!(env.store.get("key"), Some(&Object::Int(1)));
    }
}
