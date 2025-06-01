use std::collections::HashMap;

#[derive(Debug)]
pub struct Frequent {
    root: Node,
}
impl Frequent {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn add(&mut self, v: &[usize]) {
        self.root.add(v);
    }

    pub fn remove(&mut self, v: &[usize]) {
        self.root.remove(v);
    }

    pub fn contains(&mut self, v: &[usize]) -> bool {
        self.root.contains(v)
    }

    pub fn for_each(&self, f: impl FnMut(&[usize])) {
        self.root.for_each(f)
    }
}

impl Default for Frequent {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug)]
struct Node {
    map: HashMap<usize, Node>,
    is_in: bool,
}
impl Node {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            is_in: false,
        }
    }

    pub fn contains(&mut self, v: &[usize]) -> bool {
        if v.is_empty() {
            return self.is_in;
        }
        if let Some(n) = self.map.get_mut(&v[0]) {
            n.contains(&v[1..])
        } else {
            false
        }
    }
    pub fn add(&mut self, v: &[usize]) {
        if v.is_empty() {
            self.is_in = true;
            return;
        }
        if let Some(n) = self.map.get_mut(&v[0]) {
            n.add(&v[1..])
        } else {
            let mut n = Node::new();
            n.add(&v[1..]);
            self.map.insert(v[0], n);
        }
    }
    pub fn remove(&mut self, v: &[usize]) -> bool {
        if v.is_empty() {
            self.is_in = false;
            return self.map.is_empty();
        }
        if let Some(n) = self.map.get_mut(&v[0]) {
            if n.remove(&v[1..]) {
                self.map.remove(&v[0]);
                self.map.is_empty() && !self.is_in
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn for_each(&self, mut f: impl FnMut(&[usize])) {
        let mut v = Vec::new();
        self.for_each_helper(&mut f, &mut v);
    }
    fn for_each_helper(&self, f: &mut impl FnMut(&[usize]), v: &mut Vec<usize>) {
        if self.is_in {
            f(v)
        }
        for (&k, n) in self.map.iter() {
            v.push(k);
            n.for_each_helper(f, v);
            v.pop();
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::frequent::Frequent;

    #[test]
    fn test_frequent() {
        let mut frequent = Frequent::new();
        frequent.add(&[0, 1, 3]);
        assert!(frequent.contains(&[0, 1, 3]));
        assert!(!frequent.contains(&[0, 1]));
        frequent.remove(&[0, 1, 3]);
        assert!(!frequent.contains(&[0, 1, 3]));
        assert!(!frequent.contains(&[0]));

        let mut frequent = Frequent::new();
        frequent.add(&[0, 1, 3, 4]);
        frequent.add(&[0, 1, 3, 5]);
        frequent.remove(&[0, 1, 3, 4]);
        assert!(frequent.contains(&[0, 1, 3, 5]));
        frequent.remove(&[0]);
        assert!(frequent.contains(&[0, 1, 3, 5]));
    }
}
