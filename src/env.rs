use super::syntax::SYNTAX;
use super::syntax::SUBR;
use super::value::{SubrFn, SyntaxFn, Value};

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
            hash_map.insert(name.to_string(), Value::Syntax(name, SyntaxFn::new(f)));
        }
        for &(name, f) in SUBR {
            hash_map.insert(name.to_string(), Value::Subr(name, SubrFn::new(f)));
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

impl PartialEq for Env {
    fn eq(&self, other: &Env) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl ::std::fmt::Debug for Env {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "<Env>")
    }
}
