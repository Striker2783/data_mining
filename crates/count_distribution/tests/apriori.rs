mod common;

use std::{collections::HashSet, sync::Arc};

use common::{Solved, test_generic};
use count_distribution::{count_distribution::CountDistribution, count_distribution_hybrid::CountDistributionHybrid};

#[test]
fn test_cd() {
    test_generic(|t, s| {
        let a = Arc::new(t);
        let a = CountDistribution::new(a, 4, s);
        let s = a.run();
        let mut combined = HashSet::new();
        for c in s.iter() {
            for e in c.iter() {
                combined.insert(e.clone());
            }
        }
        Solved::new(combined)
    });
}
#[test]
fn test_cd_hybrid() {
    test_generic(|t, s| {
        let a = CountDistributionHybrid::new(&t, 4, s, 4);
        let s = a.run();
        let mut combined = HashSet::new();
        for c in s.iter() {
            for e in c.iter() {
                combined.insert(e.clone());
            }
        }
        Solved::new(combined)
    });
}