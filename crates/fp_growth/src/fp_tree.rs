use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type MRc<T> = Rc<RefCell<T>>;
type Map = HashMap<usize, MRc<FPNode>>;
#[derive(Debug)]
pub struct FPTree {
    root: MRc<FPNode>,
    header: Map,
    tails: Map,
    sup: u64,
}
impl FPTree {
    pub fn new(sup: u64) -> Self {
        Self {
            root: Rc::new(RefCell::new(FPNode::new(usize::MAX, None))),
            header: HashMap::new(),
            tails: HashMap::new(),
            sup,
        }
    }
    /// Gets the count of the set.
    /// 0 for no set found in the tree.
    pub fn get(&self, set: &[usize]) -> u64 {
        let mut node = Some(self.root.clone());
        let mut i = 0;
        while let Some(curr_node) = node {
            let curr_node_b = curr_node.borrow();
            if i == set.len() {
                return curr_node_b.count;
            } else {
                node = curr_node_b.children.get(&set[i]).cloned();
                i += 1;
            }
        }
        0
    }
    pub fn mine(&mut self) -> Vec<Vec<usize>> {
        let mut v = Vec::new();
        let mut v2 = Vec::new();
        self.mine_helper(&mut v, &mut v2);
        v
    }
    fn mine_helper(&mut self, sets: &mut Vec<Vec<usize>>, v: &mut Vec<usize>) {
        for (&k, node) in self.header.iter() {
            let node = node.clone();
            v.push(k);
            sets.push(v.clone());
            let mut conditional_tree = FPTree::new(self.sup);
            let mut current_node = Some(node.clone());
            let mut map = HashMap::new();
            while let Some(curr_node) = current_node {
                let prefix = FPNode::get_prefix(curr_node.clone());
                let curr_node_b = curr_node.borrow();
                current_node = curr_node_b.link.clone();
                for n in prefix {
                    map.entry(n)
                        .and_modify(|n| *n += curr_node_b.count)
                        .or_insert(curr_node_b.count);
                }
            }
            current_node = Some(node.clone());
            while let Some(curr_node) = current_node {
                let mut prefix = FPNode::get_prefix(curr_node.clone());
                let curr_node_b = curr_node.borrow();
                current_node = curr_node_b.link.clone();
                prefix.retain(|n| map.get(n).unwrap_or(&0).clone() >= self.sup);
                conditional_tree.insert_conditional(&prefix, curr_node_b.count);
            }
            conditional_tree.mine_helper(sets, v);
            v.pop();
        }
    }
    fn insert_conditional(&mut self, items: &[usize], n: u64) {
        let mut i = 0;
        let mut curr_node = Some(self.root.clone());
        while let Some(node) = curr_node {
            let mut node_b = node.borrow_mut();
            node_b.count += n;
            if i >= items.len() {
                break;
            }
            let item = items[i];
            match node_b.children.get_mut(&item) {
                Some(n) => {
                    curr_node = Some(n.clone());
                },
                None => {
                    let child =
                        Rc::new(RefCell::new(FPNode::new(item, Some(Rc::downgrade(&node)))));
                    node_b.children.insert(item.clone(), child.clone());
                    match self.tails.get_mut(&item) {
                        Some(tail) => {
                            tail.borrow_mut().link = Some(child.clone());
                            *tail = child.clone();
                        }
                        None => {
                            self.tails.insert(item, child.clone());
                            self.header.insert(item, child.clone());
                        }
                    }
                    curr_node = Some(child);
                }
            }
            i += 1;
        }
    }
    pub fn insert_transaction(&mut self, items: &[usize]) {
        self.insert_conditional(items, 1);
    }
}
#[derive(Clone, Debug)]
struct FPNode {
    item: usize,
    parent: Option<Weak<RefCell<FPNode>>>,
    count: u64,
    children: Map,
    link: Option<MRc<FPNode>>,
}

impl FPNode {
    fn new(item: usize, parent: Option<Weak<RefCell<FPNode>>>) -> Self {
        Self {
            item,
            count: 0,
            link: None,
            parent,
            children: HashMap::new(),
        }
    }
    pub fn get_prefix(s: MRc<FPNode>) -> Vec<usize> {
        let mut prefix = Vec::new();
        let mut s = s.borrow_mut();
        let mut node = s.parent.as_mut().unwrap().upgrade();
        while let Some(curr) = node {
            let curr_node = curr.borrow_mut();
            if curr_node.item == usize::MAX {
                break;
            }
            prefix.push(curr_node.item);
            if let Some(parent) = curr_node.parent.clone().take() {
                let parent_rc = parent.upgrade();
                node = parent_rc;
            } else {
                break;
            }
        }
        prefix.reverse();
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_transaction() {
        let mut fp_tree = FPTree::new(2);
        fp_tree.insert_transaction(&[1, 2, 3]);
        assert_eq!(fp_tree.get(&[1]), 1);
        assert_eq!(fp_tree.get(&[1, 2]), 1);
        assert_eq!(fp_tree.get(&[1, 2, 3]), 1);
        fp_tree.insert_transaction(&[1, 2, 4]);
        assert_eq!(fp_tree.get(&[1]), 2);
        assert_eq!(fp_tree.get(&[1, 2]), 2);
        assert_eq!(fp_tree.get(&[1, 2, 3]), 1);
        assert_eq!(fp_tree.get(&[1, 2, 4]), 1);
    }
}
