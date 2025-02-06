/// A 0-indexed item set
#[derive(Debug, Default)]
pub struct TransactionSet {
    pub transactions: Vec<Vec<usize>>,
    pub num_items: usize
}

impl TransactionSet {
    pub fn new(transactions: Vec<Vec<usize>>, num_items: usize) -> Self {
        Self { transactions, num_items }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Vec<usize>> {
        self.transactions.iter()
    }
}