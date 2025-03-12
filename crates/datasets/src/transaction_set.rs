use std::{fs::File, io::{BufRead, BufReader}, ops::{Deref, DerefMut}};

/// A 0-indexed item set
#[derive(Debug, Default)]
pub struct TransactionSet {
    pub transactions: Vec<Vec<usize>>,
    pub num_items: usize
}

impl Deref for TransactionSet {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.transactions
    }
}
impl DerefMut for TransactionSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transactions
    }
}

impl TransactionSet {
    pub fn new(transactions: Vec<Vec<usize>>, num_items: usize) -> Self {
        Self { transactions, num_items }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Vec<usize>> {
        self.transactions.iter()
    }
    pub fn from_dat(f: File) -> Self {
        let mut max = 0;
        let mut transactions = Vec::new();
        for l in BufReader::new(f).lines() {
            if l.is_err() {
                continue;
            }
            let line = l.unwrap();
            let items: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            max = (*items.iter().max().unwrap()).max(max);
            transactions.push(items);
        }
        Self { transactions, num_items: max + 1 }
    }
}