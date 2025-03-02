use std::collections::HashMap;
#[derive(Debug)]
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

    pub fn join(&mut self, i: usize, sup: u64) {
        assert!(i >= 2);
        let c = self.root.join(i - 2, sup);
        self.size += c;
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

    pub fn for_each(&self, sup: u64, mut f: impl FnMut(&[usize])) {
        let mut v = Vec::new();
        self.root.for_each(&mut v, sup, &mut f)
    }

    pub fn cleaup(&mut self, sup: u64) {
        let c = self.root.cleaup(sup);
        self.size -= c;
    }
}
#[derive(Debug)]
struct Node {
    count: u64,
    map: HashMap<usize, Node>,
    done: bool,
}
impl Node {
    fn new() -> Self {
        Node {
            count: 0,
            map: HashMap::new(),
            done: false,
        }
    }
    fn cleaup(&mut self, sup: u64) -> usize {
        let mut to_remove = Vec::new();
        let mut removed = 0;
        for (&n, node) in self.map.iter_mut() {
            if node.count < sup {
                to_remove.push(n);
                continue;
            }
            removed += node.cleaup(sup);
        }
        for n in to_remove {
            self.map.remove(&n);
        }
        removed
    }
    fn transaction_update(&mut self, v: &[usize], depth: usize, curr_i: usize) {
        if depth <= curr_i {
            self.count = self.count.saturating_add(1);
            return;
        } else if v.is_empty() || v.len() < depth - curr_i - 1 || self.done {
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
    fn for_each(&self, v: &mut Vec<usize>, sup: u64, f: &mut impl FnMut(&[usize])) {
        for (&n, node) in self.map.iter() {
            if node.count < sup {
                continue;
            }
            v.push(n);
            f(&v);
            node.for_each(v, sup, f);
            v.pop();
        }
    }
    fn join(&mut self, i: usize, sup: u64) -> usize {
        if i == 0 {
            let mut v = Vec::new();
            for (&n, node) in &mut self.map {
                if node.count >= sup {
                    v.push(n);
                }
            }
            let mut count = 0;
            for i in 0..v.len() {
                for j in (i + 1)..v.len() {
                    let n1 = v[i];
                    let n2 = v[j];
                    let max = n1.max(n2);
                    let min = n1.min(n2);
                    if self.add(&[min, max]) {
                        count += 1;
                    };
                }
            }
            return count;
        } else if self.done {
            return 0;
        }
        let mut total = 0;
        for node in self.map.values_mut() {
            if node.count < sup {
                continue;
            }
            total += node.join(i - 1, sup);
        }
        if total == 0 {
            self.done = true;
        }
        total
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
    use std::collections::HashSet;

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
        trie.join(2, 5);
        assert!(trie.contains(&[1, 4]));
        assert!(trie.contains(&[2, 4]));
        assert_eq!(trie.size(), 7);
        trie.transaction_update(&[2, 3, 4], 2);
        assert_eq!(trie.get(&[2, 4]), Some(1));
        let mut set = HashSet::new();
        trie.for_each(5, |v| {
            set.insert(v.to_vec());
        });
        assert!(set.contains(&vec![1]));
        assert!(set.contains(&vec![2]));
        assert!(set.contains(&vec![4]));
        assert!(set.len() == 3);
    }
}
