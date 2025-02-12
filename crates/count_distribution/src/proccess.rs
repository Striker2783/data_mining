use std::collections::{HashMap, HashSet};

use apriori::candidates_func::join;
use datasets::utils::nested_loops;

pub struct CDProcess<'a> {
    partition: &'a [Vec<usize>],
    c: &'a Vec<Vec<usize>>,
}

impl<'a> CDProcess<'a> {
    pub fn new(partition: &'a [Vec<usize>], c: &'a Vec<Vec<usize>>) -> Self {
        Self { partition, c }
    }
    pub fn run(self, n: usize) -> HashMap<Vec<usize>, u64> {
        let mut map = HashMap::new();
        let set: HashSet<_> = self.c.iter().collect();
        if n == 1 {
            self.first(&mut map);
            return map;
        }
        join(&self.c.iter().collect::<Vec<_>>(), |join| {
            if Self::can_be_pruned(&join, &set) {
                return;
            }
            map.insert(join, 0);
        });
        for d in self.partition {
            nested_loops(
                |v| {
                    map.entry(v).and_modify(|n| *n += 1);
                },
                d,
                n,
            );
        }
        map
    }
    fn first(&self, map: &mut HashMap<Vec<usize>, u64>) {
        for v in self.partition {
            for &n in v {
                map.entry(vec![n]).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    fn can_be_pruned(v: &[usize], set: &HashSet<&Vec<usize>>) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        for i in 0..(v.len() - 2) {
            if !set.contains(&arr) {
                return true;
            }
            arr[i] = v[i + 1];
        }
        false
    }
}
