use crate::value::Value;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

pub type EnvRef = Rc<RefCell<Env>>;

#[derive(PartialEq, Clone, Default)]
pub struct Env {
    inner: HashMap<String, Value>,
    outer: Option<EnvRef>,
}

impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hello")
    }
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_wrapped() -> EnvRef {
        Rc::new(RefCell::new(Default::default()))
    }

    pub fn branch(env: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(env.borrow().clone()))
    }

    pub fn new_inner(outer: EnvRef) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    pub fn new_inner_wrapped(outer: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Self {
            outer: Some(outer),
            ..Default::default()
        }))
    }

    pub fn from(vals: impl IntoIterator<Item = (String, Value)>) -> Self {
        Self {
            inner: vals.into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.inner.insert(k, v)
    }

    pub fn get(&self, k: &str) -> Option<Value> {
        match self.inner.get(k) {
            None => match &self.outer {
                None => None,
                Some(o) => o.borrow().get(k).clone(),
            },
            v => v.cloned(),
        }
    }
}
