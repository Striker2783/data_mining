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
                prefix.retain(|n| map.get(n).unwrap_or(&0).clone() >= self.sup + 1);
                conditional_tree.insert_conditional(&prefix, curr_node_b.count);
            }
            conditional_tree.mine_helper(sets, v);
            v.pop();
        }
    }
    fn insert_conditional(&mut self, items: &[usize], n: u64) {
        FPNode::insert_transaction(
            self.root.clone(),
            items,
            &mut self.header,
            &mut self.tails,
            n,
        );
    }
    pub fn insert_transaction(&mut self, items: &[usize]) {
        FPNode::insert_transaction(
            self.root.clone(),
            items,
            &mut self.header,
            &mut self.tails,
            1,
        );
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
            count: 1,
            link: None,
            parent,
            children: HashMap::new(),
        }
    }
    pub fn insert_transaction(
        a: MRc<FPNode>,
        items: &[usize],
        header: &mut Map,
        tails: &mut Map,
        n: u64,
    ) {
        let mut bor_a = a.borrow_mut();
        if items.is_empty() {
            bor_a.count += n;
            return;
        }
        let item = items[0];
        match bor_a.children.get_mut(&item) {
            Some(child) => {
                Self::insert_transaction(child.clone(), &items[1..], header, tails, n);
            }
            None => {
                let child = Rc::new(RefCell::new(FPNode::new(item, Some(Rc::downgrade(&a)))));
                child.borrow_mut().set_count(n);
                bor_a.children.insert(item, child.clone());
                match tails.get_mut(&item) {
                    Some(tail) => {
                        tail.borrow_mut().link = Some(child.clone());
                        *tail = child.clone();
                    }
                    None => {
                        tails.insert(item, child.clone());
                        header.insert(item, child.clone());
                    }
                }
                Self::insert_transaction(child.clone(), &items[1..], header, tails, n);
            }
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

    fn set_count(&mut self, count: u64) {
        self.count = count;
    }
}
