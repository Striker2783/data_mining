use std::collections::HashSet;

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
    fn get_candidates(&self) -> Candidates {
        let mut candidates = Candidates::new();
        candidates.run(&self);
        candidates
    }
}

#[derive(Debug, Default)]
struct Candidates {
    candidates: Vec<HashSet<Vec<usize>>>,
}

impl Candidates {
    fn new() -> Self {
        Candidates::default()
    }
    fn run(&mut self, data: &Apriori) {
        self.run_one(data);
        self.run_two(data);
    }
    fn run_one(&mut self, data: &Apriori) {
        let mut first = vec![0u64; data.data.num_items];
        for d in data.data.iter() {
            for &item in d {
                first[item] += 1;
            }
        }
        let mut v = HashSet::new();
        for (i, n) in first.into_iter().enumerate() {
            if n >= data.min_support {
                v.insert(vec![i]);
            }
        }
        self.candidates.push(v);
    }
    fn run_two(&mut self, data: &Apriori) {
        let mut second = Array2D::new(data.data.num_items);
        for d in data.data.iter() {
            for i in 0..d.len() {
                for j in 0..i {
                    second.increment(d[i], d[j]);
                }
            }
        }
        let mut v = HashSet::new();
        for (r, c, count) in second.iter() {
            if count >= data.min_support {
                v.insert(vec![r,c]);
            }
        }
        self.candidates.push(v);
    }
}

#[derive(Debug)]
struct Array2DIterator<'a, T> {
    data: &'a Array2D<T>,
    row: usize,
    col: usize,
    idx: usize,
}

impl<'a, T> Array2DIterator<'a, T> {
    fn new(data: &'a Array2D<T>) -> Self {
        Self {
            data,
            row: 1,
            col: 0,
            idx: 0,
        }
    }
}
impl<'a, T: Copy> Iterator for Array2DIterator<'a, T> {
    type Item = (usize, usize, T);
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
struct Array2D<T>(Vec<T>);
impl<T: Copy> Array2D<T> {
    fn get(&self, row: usize, col: usize) -> T {
        self.0[self.get_index(row, col)]
    }
}
impl<T: Copy + Default> Array2D<T> {
    fn new(rows: usize) -> Self {
        Array2D(vec![T::default(); (rows * (rows - 1)) / 2])
    }
}
impl<T> Array2D<T> {
    fn get_index(&self, row: usize, col: usize) -> usize {
        assert!(row != col);
        let (row, col) = if row > col { (row, col) } else { (col, row) };
        let index = (row * (row - 1)) / 2 + col;
        index
    }
    fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.0[index] = value;
    }
    fn iter(&self) -> Array2DIterator<T> {
        Array2DIterator::new(self)
    }
}
impl Array2D<u64> {
    fn increment(&mut self, row: usize, col: usize) {
        let index = self.get_index(row, col);
        self.0[index] += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, vec};

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
    #[test]
    fn test_get_candidates() {
        let example = TransactionSet::new(
            vec![
                vec![0, 1, 4],
                vec![1, 3],
                vec![1, 2],
                vec![0, 1, 3],
                vec![0, 2],
                vec![1, 2],
                vec![0, 2],
                vec![0, 1, 2, 4],
                vec![0, 1, 2],
            ],
            5,
        );
        let apriori = Apriori::new(&example, 2, 1);
        let candidates = apriori.get_candidates();
        assert!(candidates.candidates[1].contains(&vec![1, 0]));
        assert!(candidates.candidates[1].contains(&vec![2, 0]));
        assert!(candidates.candidates[1].contains(&vec![4, 0]));
        assert!(candidates.candidates[1].contains(&vec![2, 1]));
        assert!(candidates.candidates[1].contains(&vec![3, 1]));
        assert!(candidates.candidates[1].contains(&vec![4, 1]));
    }
}
