use datasets::transaction_set::TransactionSet;

use crate::candidates::Candidates;
#[derive(Debug)]
pub struct Apriori {
    min_support: u64,
}

impl Apriori {
    pub fn new(min_support: u64) -> Self {
        Self {
            min_support,
        }
    }
    pub fn run(self, data: &TransactionSet) -> Vec<Candidates> {
        let mut v = Vec::new();
        v.push(Candidates::run_one(data, self.min_support));
        for i in 2.. {
            let prev = v.last().unwrap();
            let next = prev.next(data, i, self.min_support);
            if next.data().is_empty() {
                break;
            }
            v.push(next);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::apriori::Apriori;

    #[test]
    fn test_candidates() {
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
        let apriori = Apriori::new(2);
        let result = apriori.run(&example);
        assert!(result[1].data().contains(&vec![0, 1]));
        assert!(result[1].data().contains(&vec![0, 2]));
        assert!(result[1].data().contains(&vec![0, 4]));
        assert!(result[1].data().contains(&vec![1, 2]));
        assert!(result[1].data().contains(&vec![1, 3]));
        assert!(result[1].data().contains(&vec![1, 4]));
        assert_eq!(result[1].data().len(), 6);
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].data().len(), 2);
    }
}
