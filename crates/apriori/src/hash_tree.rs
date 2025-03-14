use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Deref, DerefMut},
};

use crate::transaction_id::TransactionIdCounts;
#[derive(Debug, Default)]
pub struct AprioriHashTree(AprioriHashTreeGeneric<50>);

impl AprioriHashTree {
    pub fn new() -> Self {
        Self(AprioriHashTreeGeneric::<50>::default())
    }
}
impl Deref for AprioriHashTree {
    type Target = AprioriHashTreeGeneric<50>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AprioriHashTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// A Hash Tree for the Apriori Algorithm
/// Does not care about duplicates.
#[derive(Debug, Default)]
pub struct AprioriHashTreeGeneric<const N: usize> {
    root: HashTreeInternalNode<N>,
    length: usize,
}

impl<const N: usize> AprioriHashTreeGeneric<N> {
    pub fn new() -> Self {
        Self::default()
    }
    fn get_leaf(&self, v: &[usize]) -> Option<&HashTreeLeafNode> {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &self.root.map[(hasher.finish() as usize) % N];
        for &v in v.iter().skip(1) {
            if let Some(n) = curr {
                match n.as_ref() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v.hash(&mut hasher);
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
    fn get_leaf_mut(&mut self, v: &[usize]) -> Option<&mut HashTreeLeafNode> {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &mut self.root.map[(hasher.finish() as usize) % N];
        for v in v.iter().skip(1) {
            if let Some(n) = curr {
                match n.as_mut() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v.hash(&mut hasher);
                        curr = &mut hash_tree_internal_node.map[(hasher.finish() as usize) % N];
                    }
                    Node::Leaf(_) => return None,
                }
            } else {
                return None;
            }
        }
        if let Some(n) = curr {
            match n.as_mut() {
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
    pub fn add(&mut self, v: &[usize]) {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let hash = hasher.finish() as usize;
        let mut curr = &mut self.root.map[hash % N];
        for v in v.iter().skip(1) {
            if curr.is_none() {
                *curr = Some(Box::new(Node::Internal(HashTreeInternalNode::default())));
            }
            if let Some(n) = curr {
                match n.as_mut() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v.hash(&mut hasher);
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
        self.length += 1;
    }
    pub fn increment(&mut self, v: &[usize]) -> bool {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &mut self.root.map[(hasher.finish() as usize) % N];
        for v in v.iter().skip(1) {
            if let Some(n) = curr {
                match n.as_mut() {
                    Node::Internal(hash_tree_internal_node) => {
                        let mut hasher = DefaultHasher::new();
                        v.hash(&mut hasher);
                        curr = &mut hash_tree_internal_node.map[(hasher.finish() as usize) % N];
                    }
                    Node::Leaf(_) => return false,
                }
            } else {
                return false;
            }
        }
        if let Some(n) = curr {
            match n.as_mut() {
                Node::Internal(_) => false,
                Node::Leaf(hash_tree_leaf_node) => hash_tree_leaf_node.increment(v),
            }
        } else {
            false
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
    pub fn remove(&mut self, v: &[usize]) -> Option<(Vec<usize>, u64)> {
        let leaf = self.get_leaf_mut(v);
        if let Some(l) = leaf {
            l.remove(v)
        } else {
            None
        }
    }
    pub fn for_each_mut(&mut self, mut f: impl FnMut(&[usize], &mut u64)) {
        self.root.for_each_mut(&mut f);
    }
    pub fn iter(&self) -> HashTreeIterator<N> {
        HashTreeIterator::new(self)
    }
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
enum Node<const N: usize> {
    Internal(HashTreeInternalNode<N>),
    Leaf(HashTreeLeafNode),
}
#[derive(Debug)]
struct HashTreeInternalNode<const N: usize> {
    map: [Option<Box<Node<N>>>; N],
}

impl<const N: usize> HashTreeInternalNode<N> {
    fn for_each_mut(&mut self, f: &mut impl FnMut(&[usize], &mut u64)) {
        for n in &mut self.map {
            let Some(n) = n else { continue };
            match &mut **n {
                Node::Internal(hash_tree_internal_node) => hash_tree_internal_node.for_each_mut(f),
                Node::Leaf(hash_tree_leaf_node) => hash_tree_leaf_node.for_each_mut(f),
            }
        }
    }
}

impl<const N: usize> Default for HashTreeInternalNode<N> {
    fn default() -> Self {
        Self {
            map: [const { None }; N],
        }
    }
}

#[derive(Debug, Default)]
struct HashTreeLeafNode(Vec<(Vec<usize>, u64)>);

impl HashTreeLeafNode {
    fn increment(&mut self, v: &[usize]) -> bool {
        let f = self.0.iter_mut().find(|v2| v2.0.eq(v));
        if let Some(v) = f {
            v.1 += 1;
            true
        } else {
            false
        }
    }
    fn find(&self, v: &[usize]) -> Option<&(Vec<usize>, u64)> {
        self.0.iter().find(|v2| v2.0.eq(v))
    }
    fn for_each_mut(&mut self, f: &mut impl FnMut(&[usize], &mut u64)) {
        for (v, n) in &mut self.0 {
            f(v, n);
        }
    }
    fn find_mut(&mut self, v: &[usize]) -> Option<&mut (Vec<usize>, u64)> {
        self.0.iter_mut().find(|v2| v2.0.eq(v))
    }
    fn contains(&self, v: &[usize]) -> bool {
        self.find(v).is_some()
    }
    fn add(&mut self, v: &[usize]) {
        self.0.push((v.to_vec(), 0));
    }
    fn get_count(&self, v: &[usize]) -> Option<u64> {
        self.find(v).map(|f| f.1)
    }
    fn remove(&mut self, v: &[usize]) -> Option<(Vec<usize>, u64)> {
        for i in 0..self.0.len() {
            if v.eq(self.0[i].0.as_slice()) {
                return Some(self.0.remove(i));
            }
        }
        None
    }
}
pub struct HashTreeIterator<'a, const N: usize> {
    tree: &'a AprioriHashTreeGeneric<N>,
    outer: usize,
    stack: Vec<(&'a Node<N>, usize)>,
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
                    return Some((&hash_tree_leaf_node.0[i].0, hash_tree_leaf_node.0[i].1));
                }
            }
        }
        self.next()
    }
}

impl<'a, const N: usize> HashTreeIterator<'a, N> {
    fn new(tree: &'a AprioriHashTreeGeneric<N>) -> Self {
        Self {
            tree,
            stack: Vec::new(),
            outer: 0,
        }
    }
}
impl<const N: usize> TransactionIdCounts for AprioriHashTreeGeneric<N> {
    fn increment(&mut self, v: &[usize]) -> bool {
        self.increment(v)
    }
    
    fn len(&self) -> usize {
        self.len()
    }
    
    fn for_each(&self, mut f: impl FnMut(&[usize])) {
        self.iter().for_each(|v| {
            f(v.0)
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::hash_tree::AprioriHashTreeGeneric;

    use super::AprioriHashTree;

    #[test]
    fn test_hash_tree() {
        let mut tree = AprioriHashTree::default();
        tree.add(&[1, 2]);
        assert!(tree.contains(&[1, 2]));
        tree.increment(&[1, 2]);
        assert_eq!(tree.get_count(&[1, 2]), Some(1));
        assert!(!tree.contains(&[1, 3]));
        assert_eq!(tree.get_count(&[1, 3]), None);
        assert_eq!(tree.remove(&[1, 2]), Some((vec![1, 2], 1)));
        assert!(!tree.contains(&[1, 2]));
    }
    #[test]
    fn test_hash_tree_iterator() {
        let mut tree = AprioriHashTreeGeneric::<2>::default();
        tree.add(&[1, 2]);
        tree.increment(&[1, 2]);
        tree.add(&[1, 3]);
        let mut set = HashSet::new();
        set.insert([1, 2]);
        set.insert([1, 3]);
        for item in tree.iter() {
            assert!(set.remove(item.0));
        }
        assert!(set.is_empty());
    }
}
