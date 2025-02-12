use std::collections::{HashMap, HashSet};

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
        for i in 0..self.c.len() {
            for j in (i + 1)..self.c.len() {
                let c1 = &self.c[i];
                let c2 = &self.c[j];
                if c1[..(c1.len() - 1)] != c2[..(c1.len() - 1)] {
                    continue;
                }
                let join = if c1.last().unwrap() > c2.last().unwrap() {
                    let mut temp = c2.clone();
                    temp.push(*c1.last().unwrap());
                    temp
                } else {
                    let mut temp = c1.clone();
                    temp.push(*c2.last().unwrap());
                    temp
                };
                if Self::can_be_pruned(&join, &set) {
                    continue;
                }
                map.insert(join, 0);
            }
        }
        for d in self.partition {
            let mut stack = vec![0; n];
            Self::increment_map(&mut map, d, 0, n, &mut stack);
        }
        map
    }
    fn increment_map(
        map: &mut HashMap<Vec<usize>, u64>,
        data: &[usize],
        i: usize,
        k: usize,
        stack: &mut [usize],
    ) {
        if i == k {
            let mut v = Vec::new();
            for i in stack {
                v.push(data[*i]);
            }
            map.entry(v).and_modify(|n| *n += 1);
            return;
        }
        let start = if i == 0 { 0 } else { stack[i - 1] + 1 };
        for j in start..data.len() {
            stack[i] = j;
            Self::increment_map(map, data, i + 1, k, stack);
        }
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
