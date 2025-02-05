use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Default)]
pub struct AprioriHashTree<'a, const N: usize> {
    root: HashTreeInternalNode<'a, N>,
}

impl<'a, const N: usize> AprioriHashTree<'a, N> {
    fn get_leaf(&self, v: &[usize]) -> Option<&Box<HashTreeLeafNode<'a>>> {
        assert!(!v.is_empty());
        let mut hasher = DefaultHasher::new();
        v[0].hash(&mut hasher);
        let mut curr = &self.root.map[(hasher.finish() as usize) % N];
        for i in 1..v.len() {
            if let Some(n) = curr {
                match n {
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
            match n {
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
                *curr = Some(Node::Internal(Box::new(HashTreeInternalNode::default())))
            }
            if let Some(n) = curr {
                match n {
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
            *curr = Some(Node::Leaf(Box::new(HashTreeLeafNode::default())));
        }
        if let Some(n) = curr {
            match n {
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
                match n {
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
            match n {
                Node::Internal(_) => (),
                Node::Leaf(hash_tree_leaf_node) => hash_tree_leaf_node.increment(v),
            }
        }
    }
    fn get_count(&self, v: &[usize]) -> Option<u64> {
        let leaf = self.get_leaf(v);
        if let Some(l) = leaf {
            l.get_count(v)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Node<'a, const N: usize> {
    Internal(Box<HashTreeInternalNode<'a, N>>),
    Leaf(Box<HashTreeLeafNode<'a>>),
}
#[derive(Debug)]
struct HashTreeInternalNode<'a, const N: usize> {
    map: [Option<Node<'a, N>>; N],
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

#[cfg(test)]
mod tests {
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
}
