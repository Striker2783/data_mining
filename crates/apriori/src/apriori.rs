use datasets::transaction_set::TransactionSet;

use crate::hash_tree::AprioriHashTree;
#[derive(Debug)]
pub struct Apriori<'a> {
    data: &'a TransactionSet,
    min_support: u64,
    min_confidence: u64,
}

impl<'a> Apriori<'a> {
    pub fn new(data: &'a TransactionSet, min_support: u64, min_confidence: u64) -> Self {
        Self {
            data,
            min_support,
            min_confidence,
        }
    }
    pub fn run(self) {}
    fn get_candidates(data: &TransactionSet) -> Candidates {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Candidates {
    first: Vec<usize>,
    second: Vec<(usize, usize)>,
    trees: Vec<AprioriHashTree<50>>,
}

impl Candidates {
    fn new() -> Self {
        Candidates::default()
    }
    fn run(&mut self, data: &Apriori) {
        self.run_one(data);
    }
    fn run_one(&mut self, data: &Apriori) {
        let mut first = vec![0u64; data.data.num_items];
        for d in data.data.iter() {
            for &item in d {
                first[item] += 1;
            }
        }
        for (i, n) in first.into_iter().enumerate() {
            if n >= data.min_support {
                self.first.push(i);
            }
        }
    }
    fn run_two(&mut self, data: &Apriori) {
        let mut second = Array2D::new(data.data.num_items);
        for d in data.data.iter() {
            for i in 0..d.len() {
                for j in 0..i {
                    second.increment(i, j);
                }
            }
        }
        for (r,c,count) in second.iter() {
            if count >= data.min_support {
                self.second.push((r,c));
            }
        }
    }
}

fn sqrt(n: usize) -> usize {
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        if mid * mid <= n {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}
#[derive(Debug)]
struct Array2DIterator<'a> {
    data: &'a Array2D,
    row: usize,
    col: usize,
    idx: usize,
}

impl<'a> Array2DIterator<'a> {
    fn new(data: &'a Array2D) -> Self {
        Self {
            data,
            row: 1,
            col: 0,
            idx: 0,
        }
    }
}
impl<'a> Iterator for Array2DIterator<'a> {
    type Item = (usize, usize, u64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.0.len() {
            return None;
        }
        let element = (self.row, self.col, self.data.0[self.idx]);
        self.idx += 1;
        self.col += 1;
        if self.col >= self.row {
            self.col = 0;
            self.row += 1;
        }
        Some(element)
    }
}

#[derive(Debug, Default)]
struct Array2D(Vec<u64>);
impl Array2D {
    fn new(rows: usize) -> Self {
        Array2D(vec![0; (rows * (rows - 1)) / 2])
    }
    fn get_index(&self, row: usize, col: usize) -> usize {
        assert!(row != col);
        let (row, col) = if row > col { (row, col) } else { (col, row) };
        let index = (row * (row - 1)) / 2 + col;
        index
    }
    fn get(&self, row: usize, col: usize) -> u64 {
        self.0[self.get_index(row, col)]
    }
    fn increment(&mut self, row: usize, col: usize) {
        let index = self.get_index(row, col);
        self.0[index] += 1;
    }
    fn set(&mut self, row: usize, col: usize, value: u64) {
        let index = self.get_index(row, col);
        self.0[index] = value;
    }
    fn iter(&self) -> Array2DIterator {
        Array2DIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array2d() {
        let mut array2d = Array2D::new(3);
        array2d.increment(0, 1);
        assert_eq!(array2d.get(0, 1), 1);
        array2d.increment(1, 2);
        assert_eq!(array2d.get(1, 2), 1);
        let mut array2d = Array2D::new(5);
        array2d.increment(4, 3);
        array2d.increment(4, 3);
        assert_eq!(array2d.get(4, 3), 2);
        let mut array2d = Array2D::new(10);
        let mut count = 0;
        for i in 0..10 {
            for j in 0..i {
                array2d.set(i, j, count);
                count += 1;
            }
        }
        for i in 0..45 {
            assert_eq!(array2d.0[i], i as u64);
        }
        for (i, e) in array2d.iter().enumerate() {
            assert_eq!(e.2, i as u64);
            assert_eq!(array2d.get(e.0, e.1), e.2);
        }
    }
}
