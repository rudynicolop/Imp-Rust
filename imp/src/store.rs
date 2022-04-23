use std::collections::HashMap;

pub struct Store(HashMap<String, i32>);

impl Store {
    pub fn new () -> Self { Store(HashMap::new()) }
    
    pub fn get(&self, var: &str) -> Option<i32> {
        self.0.get(var).copied()
    }

    pub fn insert(&mut self, var: &str, value: i32) {
        self.0.insert(String::from(var), value);
    }
}
