use std::collections::HashMap;
use crate::value::Value;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Env {
    inner: HashMap<String, Value>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_inner(outer: Env) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            ..Default::default()
        }
    }

    pub fn from(vals: impl Into<Vec<(String, Value)>>) -> Self {
        Self {
            inner: vals.into().into_iter().collect(),
            outer: None,
        }
    }

    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.inner.insert(k, v)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if let Some(v) = self.inner.get(key) {
            return Some(v)
        }

        if let Some(v) = self.outer.as_ref()?.get(key) {
            return Some(v)
        }

        None
    }
}
