use crate::builtin::{SUBR, SYNTAX};
use crate::value::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ChainMapCell<T: Clone> {
    inner: HashMap<String, T>,
    outer: Option<ChainMap<T>>,
}

/// An extended hash map with lookup delegation, like a JavaScript object.
pub struct ChainMap<T: Clone>(Rc<RefCell<ChainMapCell<T>>>);

impl<T: Clone> ChainMap<T> {
    pub fn new(outer: Option<ChainMap<T>>) -> ChainMap<T> {
        ChainMap(Rc::new(RefCell::new(ChainMapCell {
            inner: HashMap::new(),
            outer: outer,
        })))
    }

    pub fn extend(&self) -> ChainMap<T> {
        Self::new(Some(ChainMap(self.0.clone())))
    }

    pub fn insert(&self, key: String, value: T) {
        self.0.borrow_mut().inner.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<T> {
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
}

impl<T: Clone> Clone for ChainMap<T> {
    fn clone(&self) -> ChainMap<T> {
        ChainMap(self.0.clone())
    }
}

impl<T: Clone> PartialEq for ChainMap<T> {
    fn eq(&self, other: &ChainMap<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Clone> ::std::fmt::Debug for ChainMap<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let ptr = Rc::into_raw(self.0.clone());
        let ret = write!(f, "<Env {}>", ptr as usize);
        let _ = unsafe { Rc::from_raw(ptr) };
        ret
    }
}

/// A hash map contains lisp variables.
pub type Env = ChainMap<Value>;

impl Env {
    /// Create a new namespace with builtin variables.
    pub fn new_default() -> Env {
        let env = Env::new(None);
        for &(name, f) in SYNTAX {
            env.insert(name.to_string(), Value::Syntax(name, f));
        }
        for &(name, f) in SUBR {
            env.insert(name.to_string(), Value::Subr(name, f));
        }
        env
    }

    /// Print variables.
    pub fn print(&self) {
        println!("{:?}", self.0.borrow().inner);
    }
}
