use std::collections::HashSet;

use datasets::transaction_set::TransactionSet;

use crate::array2d::Array2D;
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
                v.insert(vec![r, c]);
            }
        }
        self.candidates.push(v);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

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
