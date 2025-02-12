use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashTable {
    arr: Vec<u64>,
}

impl HashTable {
    pub fn new(n: usize) -> Self {
        Self { arr: vec![0; n] }
    }
    pub fn increment(&mut self, v: &[usize]) {
        *self.get_mut(v) += 1;
    }
    pub fn get(&self, v: &[usize]) -> u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let hash = hasher.finish();
        let len = self.arr.len();
        self.arr[(hash as usize) % len]
    }
    fn get_mut(&mut self, v: &[usize]) -> &mut u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let hash = hasher.finish();
        let len = self.arr.len();
        &mut self.arr[(hash as usize) % len]
    }
}