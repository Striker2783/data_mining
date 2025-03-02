use std::collections::HashMap;

pub struct AprioriTrie {
    root: Node,
    size: usize,
}

impl AprioriTrie {
    pub fn new() -> Self {
        let mut n = Node::new();
        n.count = u64::MAX;
        Self { root: n, size: 0 }
    }

    pub fn contains(&self, v: &[usize]) -> bool {
        self.get(v).is_some()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, v: &[usize]) -> Option<u64> {
        self.root.get(v)
    }

    pub fn add(&mut self, v: &[usize]) -> bool {
        if self.root.add(v) {
            self.size += 1;
            true
        } else {
            false
        }
    }

    pub fn insert(&mut self, v: &[usize], n: u64) {
        match self.root.get_mut(v) {
            Some(a) => *a = n,
            None => {
                self.add(v);
                *self.root.get_mut(v).unwrap() = n;
            }
        }
    }

    pub fn transaction_update(&mut self, v: &[usize], depth: usize) {
        self.root.transaction_update(v, depth, 0)
    }
}

struct Node {
    count: u64,
    map: HashMap<usize, Node>,
}
impl Node {
    fn new() -> Self {
        Node {
            count: 0,
            map: HashMap::new(),
        }
    }
    fn transaction_update(&mut self, v: &[usize], depth: usize, curr_i: usize) {
        if depth == curr_i {
            self.count = self.count.saturating_add(1);
            return;
        } else if v.is_empty() {
            return;
        }
        for i in 0..(v.len() - (depth - curr_i - 1)) {
            let n = v[i];
            match self.map.get_mut(&n) {
                Some(a) => a.transaction_update(&v[(i + 1)..], depth, curr_i + 1),
                None => (),
            }
        }
    }
    fn add(&mut self, v: &[usize]) -> bool {
        if v.is_empty() {
            return false;
        }
        match self.map.get_mut(&v[0]) {
            Some(n) => n.add(&v[1..]),
            None => {
                let mut n = Self::new();
                n.add(&v[1..]);
                self.map.insert(v[0], n);
                return true;
            }
        }
    }
    fn get(&self, v: &[usize]) -> Option<u64> {
        if v.is_empty() {
            return Some(self.count);
        }
        if let Some(k) = self.map.get(&v[0]) {
            return k.get(&v[1..]);
        } else {
            return None;
        }
    }
    fn get_mut(&mut self, v: &[usize]) -> Option<&mut u64> {
        if v.is_empty() {
            return Some(&mut self.count);
        }
        if let Some(k) = self.map.get_mut(&v[0]) {
            return k.get_mut(&v[1..]);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AprioriTrie;

    #[test]
    fn test_trie() {
        let mut trie = AprioriTrie::new();
        assert_eq!(trie.size(), 0);
        assert_eq!(trie.get(&[0]), None);
        assert!(trie.add(&[1]));
        assert!(!trie.add(&[1]));
        assert_eq!(trie.get(&[1]), Some(0));
        trie.insert(&[1], 1);
        assert_eq!(trie.get(&[1]), Some(1));
        trie.add(&[0, 1, 2, 3]);
        assert!(trie.contains(&[0]));
        assert!(trie.contains(&[0, 1]));
        assert!(trie.contains(&[0, 1, 2]));
        assert!(trie.contains(&[0, 1, 2, 3]));
    }
    #[test]
    fn test_transaction_update() {
        let mut trie = AprioriTrie::new();
        trie.insert(&[1], 10);
        trie.insert(&[2], 20);
        trie.insert(&[3], 1);
        trie.insert(&[4], 10);
        trie.add(&[1, 2]);
        trie.transaction_update(&[1, 2, 3], 2);
        assert_eq!(trie.get(&[1, 2]), Some(1));
        assert_eq!(trie.size(), 5);
    }
}
