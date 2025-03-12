use std::collections::{HashMap, HashSet};

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::candidates_func::join;
#[derive(Debug, Default)]
pub struct TransactionIDs {
    v: Vec<TransactionID>,
}

impl TransactionIDs {
    pub fn new(v: Vec<TransactionID>) -> Self {
        Self { v }
    }
    pub fn count_with_next<T: TransactionIdCounts>(&self, set: &mut T) -> Self {
        let mut o = Self::default();
        for d in &self.v {
            let a = d.count_with_next(set);
            o.v.push(a);
        }
        o
    }
    pub fn count<T: TransactionIdCounts>(&self, set: &mut T) {
        for d in &self.v {
            d.count(set);
        }
    }
    pub fn from_prev(&self, set: &HashSet<Vec<usize>>) -> TransactionIDs {
        let mut v = Vec::new();
        for d in &self.v {
            let value = d.from_prev(set);
            if value.ids().is_empty() {
                continue;
            }
            v.push(value);
        }
        Self::new(v)
    }
    pub fn start(data: &Vec<Vec<usize>>) -> TransactionIDs {
        let mut v = Vec::new();
        for d in data {
            let value = TransactionID::start(d);
            if value.ids().is_empty() {
                continue;
            }
            v.push(value);
        }
        Self::new(v)
    }
    pub fn from_transaction(data: &Vec<Vec<usize>>, k: usize, set: &HashSet<Vec<usize>>) -> Self {
        let mut v = Vec::new();
        for d in data {
            let value = TransactionID::from_transaction(d, k, set);
            if value.ids().is_empty() {
                continue;
            }
            v.push(value);
        }
        Self::new(v)
    }
}
impl From<&TransactionSet> for TransactionIDs {
    fn from(transaction_set: &TransactionSet) -> Self {
        Self::start(&transaction_set.transactions)
    }
}

#[derive(Debug, Default)]
pub struct TransactionID {
    v: HashSet<Vec<usize>>,
}

impl TransactionID {
    pub fn new(v: HashSet<Vec<usize>>) -> Self {
        Self { v }
    }

    pub fn count_with_next<T: TransactionIdCounts>(&self, set: &mut T) -> Self {
        if set.len() < self.ids().len() {
            let mut t = TransactionID::default();
            set.for_each(|v| {
                let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
                if !self.ids().contains(&arr) {
                    return;
                }
                for i in 0..arr.len() {
                    arr[i] = v[i];
                    if !self.ids().contains(&arr) {
                        return;
                    }
                }
                t.ids_mut().insert(v.to_vec());
            });
            for v in t.ids() {
                set.increment(v);
            }
            t
        } else {
            let mut o = TransactionID::default();
            join(&self.v.iter().collect::<Vec<_>>(), |curr| {
                if set.increment(&curr) {
                    o.v.insert(curr);
                }
            });
            o
        }
    }

    pub fn count<T: TransactionIdCounts>(&self, set: &mut T) {
        join(&self.v.iter().collect::<Vec<_>>(), |curr| {
            set.increment(&curr);
        });
    }
    pub fn from_prev(&self, set: &HashSet<Vec<usize>>) -> Self {
        let mut v = HashSet::new();
        join(&self.v.iter().collect::<Vec<_>>(), |curr| {
            if set.contains(&curr) {
                v.insert(curr);
            }
        });
        Self { v }
    }
    pub fn start(data: &[usize]) -> Self {
        Self::new(data.iter().cloned().map(|n| vec![n]).collect())
    }
    pub fn from_transaction(data: &[usize], k: usize, set: &HashSet<Vec<usize>>) -> Self {
        if data.len() < k {
            return Self::default();
        }
        if set.len() < 400 {
            let setdata: HashSet<_> = data.iter().cloned().collect();
            let mut output = HashSet::new();
            for s in set {
                if s.iter().all(|n| setdata.contains(n)) {
                    output.insert(s.to_vec());
                }
            }
            Self { v: output }
        } else {
            let mut output = HashSet::new();
            nested_loops(
                |a| {
                    if set.contains(a) {
                        output.insert(a.to_vec());
                    }
                },
                data,
                k,
            );
            Self { v: output }
        }
    }
    pub fn ids(&self) -> &HashSet<Vec<usize>> {
        &self.v
    }
    pub fn ids_mut(&mut self) -> &mut HashSet<Vec<usize>> {
        &mut self.v
    }
}

pub trait TransactionIdCounts {
    fn increment(&mut self, v: &[usize]) -> bool;
    fn len(&self) -> usize;
    fn for_each(&self, f: impl FnMut(&[usize]));
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl TransactionIdCounts for HashMap<Vec<usize>, u64> {
    fn increment(&mut self, v: &[usize]) -> bool {
        if let Some(a) = self.get_mut(v) {
            *a += 1;
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn for_each(&self, mut f: impl FnMut(&[usize])) {
        self.iter().for_each(|v| f(v.0));
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_transaction_id() {
        let transaction = TransactionID::start(&[1, 2, 3, 5]);
        for n in [1, 2, 3, 5] {
            assert!(transaction.ids().contains(&vec![n]));
        }
        let mut map = HashMap::new();
        for v in [
            vec![1, 2],
            vec![1, 3],
            vec![1, 5],
            vec![2, 3],
            vec![2, 5],
            vec![3, 5],
            vec![1, 4],
        ] {
            map.insert(v, 0);
        }
        transaction.count(&mut map);
        assert_eq!(map[&vec![1, 2]], 1);
        assert_eq!(map[&vec![1, 3]], 1);
        assert_eq!(map[&vec![1, 5]], 1);
        assert_eq!(map[&vec![1, 4]], 0);
        let set = map
            .into_iter()
            .filter_map(|(k, v)| if v > 0 { Some(k) } else { None })
            .collect::<HashSet<_>>();
        let next = transaction.from_prev(&set);
        assert!(next.ids().contains(&vec![1, 3]));
        assert_eq!(next.ids().len(), 6);
    }
}
