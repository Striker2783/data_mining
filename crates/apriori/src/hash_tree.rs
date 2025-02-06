use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Default)]
pub struct AprioriHashTree<'a, const N: usize> {
    root: HashTreeInternalNode<'a, N>,
}

impl<'a, const N: usize> AprioriHashTree<'a, N> {
    fn get_leaf(&self, v: &[usize]) -> Option<&HashTreeLeafNode<'a>> {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &self.root.map[(hasher.finish() as usize) % N];
        for i in 1..v.len() {
            if let Some(n) = curr {
                match n.as_ref() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v[i].hash(&mut hasher);
                        curr = &hash_tree_internal_node.map[(hasher.finish() as usize) % N];
                    }
                    Node::Leaf(_) => return None,
                }
            } else {
                return None;
            }
        }
        if let Some(n) = curr {
            match n.as_ref() {
                Node::Internal(_) => return None,
                Node::Leaf(hash_tree_leaf_node) => return Some(hash_tree_leaf_node),
            }
        }
        None
    }
    pub fn contains(&self, v: &[usize]) -> bool {
        assert!(!v.is_empty());
        let leaf = self.get_leaf(v);
        if let Some(l) = leaf {
            l.contains(v)
        } else {
            false
        }
    }
    pub fn add(&mut self, v: &'a [usize]) {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let hash = hasher.finish() as usize;
        let mut curr = &mut self.root.map[hash % N];
        for i in 1..v.len() {
            if curr.is_none() {
                *curr = Some(Box::new(Node::Internal(HashTreeInternalNode::default())));
            }
            if let Some(n) = curr {
                match n.as_mut() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v[i].hash(&mut hasher);
                        curr = &mut hash_tree_internal_node.map[(hasher.finish() as usize) % N];
                    }
                    Node::Leaf(_) => return,
                }
            }
        }
        if curr.is_none() {
            *curr = Some(Box::new(Node::Leaf(HashTreeLeafNode::default())));
        }
        if let Some(n) = curr {
            match n.as_mut() {
                Node::Internal(_) => (),
                Node::Leaf(hash_tree_leaf_node) => hash_tree_leaf_node.add(v),
            }
        }
    }
    pub fn increment(&mut self, v: &[usize]) {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &mut self.root.map[(hasher.finish() as usize) % N];
        for i in 1..v.len() {
            if let Some(n) = curr {
                match n.as_mut() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v[i].hash(&mut hasher);
                        curr = &mut hash_tree_internal_node.map[(hasher.finish() as usize) % N];
                    }
                    Node::Leaf(_) => return,
                }
            } else {
                return;
            }
        }
        if let Some(n) = curr {
            match n.as_mut() {
                Node::Internal(_) => (),
                Node::Leaf(hash_tree_leaf_node) => hash_tree_leaf_node.increment(v),
            }
        }
    }
    pub fn get_count(&self, v: &[usize]) -> Option<u64> {
        let leaf = self.get_leaf(v);
        if let Some(l) = leaf {
            l.get_count(v)
        } else {
            None
        }
    }
    pub fn iter(&'a self) -> HashTreeIterator<'a, N> {
        HashTreeIterator::new(self)
    }
}

#[derive(Debug)]
enum Node<'a, const N: usize> {
    Internal(HashTreeInternalNode<'a, N>),
    Leaf(HashTreeLeafNode<'a>),
}
#[derive(Debug)]
struct HashTreeInternalNode<'a, const N: usize> {
    map: [Option<Box<Node<'a, N>>>; N],
}

impl<const N: usize> Default for HashTreeInternalNode<'_, N> {
    fn default() -> Self {
        Self {
            map: [const { None }; N],
        }
    }
}

#[derive(Debug, Default)]
struct HashTreeLeafNode<'a>(Vec<(&'a [usize], u64)>);

impl<'a> HashTreeLeafNode<'a> {
    fn increment(&mut self, v: &[usize]) {
        let f = self.0.iter_mut().find(|v2| v2.0.eq(v));
        if let Some(v) = f {
            v.1 += 1;
        }
    }
    fn contains(&self, v: &[usize]) -> bool {
        let f = self.0.iter().find(|v2| v2.0.eq(v));
        f.is_some()
    }
    fn add(&mut self, v: &'a [usize]) {
        self.0.push((v, 0));
    }
    fn get_count(&self, v: &[usize]) -> Option<u64> {
        let f = self.0.iter().find(|v2| v2.0.eq(v));
        f.map(|f| f.1)
    }
}
pub struct HashTreeIterator<'a, const N: usize> {
    tree: &'a AprioriHashTree<'a, N>,
    outer: usize,
    stack: Vec<(&'a Node<'a, N>, usize)>,
}

impl<'a, const N: usize> Iterator for HashTreeIterator<'a, N> {
    type Item = (&'a [usize], u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            let mut i = self.outer;
            while i < N && self.tree.root.map[i].is_none() {
                i += 1;
            }
            if i >= N {
                return None;
            }
            self.outer = i + 1;
            match &self.tree.root.map[i] {
                Some(a) => self.stack.push((a.as_ref(), 0)),
                None => unreachable!(),
            }
        }
        while !self.stack.is_empty() {
            let mut i = self.stack.last().unwrap().1;
            match self.stack.last().unwrap().0 {
                Node::Internal(hash_tree_internal_node) => {
                    while i < N && hash_tree_internal_node.map[i].is_none() {
                        i += 1;
                    }
                    if i >= N {
                        self.stack.pop();
                        continue;
                    }
                    self.stack.last_mut().unwrap().1 = i + 1;
                    match &hash_tree_internal_node.map[i] {
                        Some(a) => self.stack.push((a, 0)),
                        None => unreachable!(),
                    }
                }
                Node::Leaf(hash_tree_leaf_node) => {
                    if i >= hash_tree_leaf_node.0.len() {
                        self.stack.pop();
                        continue;
                    }
                    self.stack.last_mut().unwrap().1 += 1;
                    return Some(hash_tree_leaf_node.0[i]);
                }
            }
        }
        self.next()
    }
}

impl<'a, const N: usize> HashTreeIterator<'a, N> {
    fn new(tree: &'a AprioriHashTree<'a, N>) -> Self {
        Self {
            tree,
            stack: Vec::new(),
            outer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::AprioriHashTree;

    #[test]
    fn test_hash_tree() {
        let mut tree = AprioriHashTree::<50>::default();
        tree.add(&[1, 2]);
        assert!(tree.contains(&[1, 2]));
        tree.increment(&[1, 2]);
        assert_eq!(tree.get_count(&[1, 2]), Some(1));
        assert!(!tree.contains(&[1, 3]));
        assert_eq!(tree.get_count(&[1, 3]), None);
    }
    #[test]
    fn test_hash_tree_iterator() {
        let mut tree = AprioriHashTree::<2>::default();
        tree.add(&[1, 2]);
        tree.increment(&[1, 2]);
        tree.add(&[1, 3]);
        let mut set = HashSet::new();
        set.insert([1,2]);
        set.insert([1,3]);
        for item in tree.iter() {
            assert!(set.remove(item.0));
        }
        assert!(set.is_empty());
    }
}
