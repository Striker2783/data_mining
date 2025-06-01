use std::collections::HashMap;
#[derive(Debug)]
pub struct Trie {
    root: Node,
}
impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new(0) }
    }
    pub fn initial_groups(&mut self, v: &[u64], sup: u64) {
        for i in 0..v.len() {
            if v[i] < sup {
                continue;
            }
            self.root.insert(&[i], v[i]);
            for j in (i + 1)..v.len() {
                if v[j] < sup {
                    continue;
                }
                self.root.add(&[i, j]);
            }
        }
    }
    pub fn count(&mut self, data: &[usize], i: usize) {
        self.root.count(data, i);
    }
    pub fn get_count(&self, v: &[usize]) -> Option<u64> {
        Some(self.root.get(v)?.count)
    }
    pub fn get_tail_count(&self, v: &[usize]) -> Option<u64> {
        Some(self.root.get(v)?.tail_count)
    }
    pub fn contains(&self, v: &[usize]) -> bool {
        self.root.get(v).is_some()
    }
    pub fn count_frequent(&mut self, i: usize, f: impl FnMut(&[usize]), sup: u64) {
        self.root.count_frequent(f, i, sup);
    }

    pub fn for_each_tails(&self, f: impl FnMut(&[usize]), i: usize) {
        self.root.for_each_tails(f, i)
    }
    ///
    /// The closure returns true to retain the set
    pub fn tails_filter(&mut self, f: impl FnMut(&[usize]) -> bool, i: usize) {
        self.root.tails_filter(f, i)
    }

    pub fn add(&mut self, v: &[usize]) {
        self.root.add(v)
    }

    pub fn insert(&mut self, v: &[usize], n: u64) {
        self.root.insert(v, n)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug)]
struct Node {
    head: usize,
    count: u64,
    tail_count: u64,
    tails: HashMap<usize, Node>,
}

impl Node {
    fn new(head: usize) -> Self {
        Self {
            head,
            count: 0,
            tail_count: 0,
            tails: HashMap::new(),
        }
    }
    pub fn remove(&mut self, v: &[usize]) -> bool {
        if v.is_empty() {
            return true;
        }
        if let Some(n) = self.tails.get_mut(&v[0]) {
            if n.remove(&v[1..]) {
                self.tails.remove(&v[0]);
            }
        }
        self.tails.is_empty()
    }
    pub fn tails_filter(&mut self, mut f: impl FnMut(&[usize]) -> bool, i: usize) {
        let mut v = Vec::new();
        self.tails_filter_helper(&mut f, &mut v, i);
    }
    /// Returns true to retain
    fn tails_filter_helper(
        &mut self,
        f: &mut impl FnMut(&[usize]) -> bool,
        v: &mut Vec<usize>,
        i: usize,
    ) -> bool {
        if i == 0 {
            if self.tails.is_empty() {
                return false;
            }
            let mut map: Vec<_> = self.tails.keys().copied().collect();
            map.sort_unstable();
            for n in map.iter().cloned() {
                v.push(n);
            }
            let b = f(v);
            for _ in 0..map.len() {
                v.pop();
            }
            return b;
        }
        self.tails.retain(|&k, n| {
            v.push(k);
            let b = n.tails_filter_helper(f, v, i - 1);
            v.pop();
            b
        });
        !self.tails.is_empty()
    }
    pub fn for_each_tails(&self, mut f: impl FnMut(&[usize]), i: usize) {
        let mut v = Vec::new();
        self.for_each_tails_helper(&mut f, &mut v, i);
    }
    fn for_each_tails_helper(&self, f: &mut impl FnMut(&[usize]), v: &mut Vec<usize>, i: usize) {
        if i == 0 {
            if self.tails.is_empty() {
                return;
            }
            let mut map: Vec<_> = self.tails.keys().copied().collect();
            map.sort_unstable();
            for n in map.iter().cloned() {
                v.push(n);
            }
            f(v);
            for _ in 0..map.len() {
                v.pop();
            }
            return;
        }
        for (_, n) in self.tails.iter() {
            n.for_each_tails_helper(f, v, i - 1);
        }
    }
    pub fn count_frequent(&mut self, mut f: impl FnMut(&[usize]), i: usize, sup: u64) {
        let mut stack = Vec::new();
        self.count_frequent_helper(&mut f, &mut stack, i, sup);
    }
    fn count_frequent_helper(
        &mut self,
        f: &mut impl FnMut(&[usize]),
        v: &mut Vec<usize>,
        i: usize,
        sup: u64,
    ) {
        if i == 0 {
            self.tails.retain(|_, n| n.count >= sup);
            let mut next: Vec<_> = self.tails.iter().map(|(&k, _)| k).collect();
            next.sort_unstable();
            if self.tail_count >= sup {
                for &k in next.iter() {
                    v.push(k);
                }
                f(v);
                for _ in 0..next.len() {
                    v.pop();
                }
                return;
            }
            for (i, &i1) in next.iter().enumerate() {
                for &j1 in next.iter().skip(i + 1) {
                    self.add(&[i1, j1]);
                }
                let node = self.tails.get(&i1).unwrap();
                if node.lower_bound(self) >= sup {
                    for &j1 in next.iter().skip(i + 1) {
                        self.remove(&[i1, j1]);
                    }
                    v.push(i1);
                    for &j1 in next.iter().skip(i + 1) {
                        v.push(j1);
                    }
                    f(v);
                    for _ in (i + 1)..next.len() {
                        v.pop();
                    }
                    v.pop();
                    continue;
                }
            }
            if next.is_empty() {
                f(v)
            } else {
                v.push(*next.last().unwrap());
                f(v);
                v.pop();
            }
            return;
        }
        for (_, n) in self.tails.iter_mut() {
            v.push(n.head);
            n.count_frequent_helper(f, v, i - 1, sup);
            v.pop();
        }
    }
    fn lower_bound(&self, itemset: &Node) -> u64 {
        let mut sum = 0;
        for (_, n) in self.tails.iter() {
            sum += itemset.count - itemset.tails.get(&n.head).unwrap().count;
        }
        self.count.saturating_sub(sum)
    }
    fn get(&self, v: &[usize]) -> Option<&Self> {
        if v.is_empty() {
            return Some(self);
        }
        self.tails.get(&v[0])?.get(&v[1..])
    }
    fn get_mut(&mut self, v: &[usize]) -> Option<&mut Self> {
        if v.is_empty() {
            return Some(self);
        }
        self.tails.get_mut(&v[0])?.get_mut(&v[1..])
    }
    fn add(&mut self, v: &[usize]) {
        self.insert(v, 0);
    }
    pub fn insert(&mut self, v: &[usize], n: u64) {
        if v.is_empty() {
            self.count = n;
            return;
        }
        match self.tails.get_mut(&v[0]) {
            Some(n) => n.add(&v[1..]),
            None => {
                let mut node = Self::new(v[0]);
                node.insert(&v[1..], n);
                self.tails.insert(v[0], node);
            }
        }
    }
    fn count(&mut self, transaction: &[usize], i: usize) {
        if i == 0 {
            let mut increments = 0;
            for &n in transaction {
                if let Some(n) = self.tails.get_mut(&n) {
                    n.count += 1;
                    increments += 1;
                }
            }
            if increments == self.tails.len() {
                self.tail_count += 1;
            }
            return;
        }
        for (j, &n) in transaction.iter().enumerate() {
            if let Some(node) = self.tails.get_mut(&n) {
                node.count(&transaction[(j + 1)..], i - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Trie;

    #[test]
    fn test_initial() {
        let items = vec![1, 1, 0, 1];
        let mut trie = Trie::new();
        trie.initial_groups(&items, 1);
        assert!(trie.contains(&[0, 1]));
        assert!(trie.contains(&[0, 3]));
        assert!(trie.contains(&[1, 3]));
        trie.root.count(&[0, 1, 3], 1);
        assert_eq!(trie.get_count(&[0, 1]), Some(1));
        assert_eq!(trie.get_tail_count(&[0]), Some(1));
    }

    #[test]
    fn test_tails_filter() {
        let mut trie = Trie::new();
        trie.add(&[1, 2, 3, 5]);
        trie.add(&[1, 2, 3, 4]);
        trie.add(&[1, 2, 4, 5]);
        trie.add(&[1, 2, 4, 6]);
        trie.tails_filter(
            |v| {
                if v == [1, 2, 4, 5, 6] {
                    return false;
                }
                true
            },
            3,
        );
        assert!(trie.contains(&[1, 2, 3]));
        assert!(!trie.contains(&[1, 2, 4]));
    }
}
