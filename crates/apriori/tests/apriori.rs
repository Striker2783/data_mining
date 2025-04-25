use std::collections::HashSet;

use apriori::{
    apriori::Apriori, apriori_hybrid::AprioriHybrid, apriori_tid::AprioriTID,
    apriori_trie::AprioriT,
};
use tester::test_utils::{Solved, test_generic};

#[test]
fn test_apriori() {
    test_generic(|t, s| {
        let a = Apriori::new(s);
        let s = a.run(&t);
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
fn test_tid() {
    test_generic(|t, s| {
        let a = AprioriTID::new(s);
        let s = a.run(&t);
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
fn test_hybrid() {
    test_generic(|t, s| {
        let a = AprioriHybrid::new(s, 4);
        let s = a.run(&t);
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
fn test_trie() {
    test_generic(|t, s| {
        let mut a = AprioriT::new(s);
        a.run(&t);
        let mut combined = HashSet::new();
        a.trie().for_each(s, |e| {
            combined.insert(e.to_vec());
        });
        Solved::new(combined)
    });
}
