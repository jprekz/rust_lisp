use super::parser::Value;
use super::syntax::SYNTAX;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct EnvCell {
    inner: HashMap<String, Value>,
    outer: Option<Env>,
}

pub struct Env(Rc<RefCell<EnvCell>>);

impl Env {
    pub fn new_default() -> Env {
        let mut hash_map = HashMap::new();
        for &(name, f) in SYNTAX {
            hash_map.insert(name.to_string(), Value::Syntax(name, f));
        }
        Env::new(EnvCell {
            inner: hash_map,
            outer: None,
        })
    }

    pub fn extend(&self) -> Env {
        Env::new(EnvCell {
            inner: HashMap::new(),
            outer: Some(Env(self.0.clone())),
        })
    }

    pub fn insert(&self, key: String, value: Value) {
        self.0.borrow_mut().inner.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<Value> {
        if let Some(value) = self.0.borrow().inner.get(&key) {
            Some(value.clone())
        } else {
            self.0
                .borrow()
                .outer
                .as_ref()
                .and_then(|outer| outer.get(key))
        }
    }

    fn new(env_inner: EnvCell) -> Env {
        Env(Rc::new(RefCell::new(env_inner)))
    }
}

impl Clone for Env {
    fn clone(&self) -> Env {
        Env(self.0.clone())
    }
}

