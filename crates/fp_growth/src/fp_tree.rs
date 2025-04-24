use std::{cell::RefCell, collections::HashMap, rc::Rc};

type MRc<T> = Rc<RefCell<T>>;
type Map = HashMap<usize, MRc<FPNode>>;
#[derive(Debug)]
pub struct FPTree {
    root: MRc<FPNode>,
    header: Map,
    tails: Map,
}
impl FPTree {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(FPNode::new(usize::MAX))),
            header: HashMap::new(),
            tails: HashMap::new(),
        }
    }
    pub fn insert_transaction(&mut self, items: &[usize]) {
        self.root
            .borrow_mut()
            .insert_transaction(items, &mut self.header, &mut self.tails);
    }
}
#[derive(Clone, Debug)]
struct FPNode {
    item: usize,
    count: u64,
    children: Map,
    link: Option<MRc<FPNode>>,
}

impl FPNode {
    fn new(item: usize) -> Self {
        Self {
            item,
            count: 0,
            link: None,
            children: HashMap::new(),
        }
    }
    pub fn insert_transaction(&mut self, items: &[usize], header: &mut Map, tails: &mut Map) {
        if items.is_empty() {
            self.count += 1;
            return;
        }
        let item = items[0];
        match self.children.get_mut(&item) {
            Some(child) => {
                child
                    .borrow_mut()
                    .insert_transaction(&items[1..], header, tails);
            }
            None => {
                let child = Rc::new(RefCell::new(FPNode::new(item)));
                self.children.insert(item, child.clone());
                match tails.get_mut(&item) {
                    Some(tail) => {
                        tail.borrow_mut().link = Some(child.clone());
                        *tail = child.clone();
                    },
                    None => {
                        tails.insert(item, child.clone());
                        header.insert(item, child.clone());
                    }
                }
            }
        }
    }
}
