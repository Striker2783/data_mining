use fp_growth::fp_growth::FPGrowth;
use tester::test_utils::{Solved, test_generic};

#[test]
fn test_fp_growth() {
    test_generic(|set, s| {
        let fp_growth = FPGrowth::new(s, set);
        let result = fp_growth.run();
        let result = result
            .into_iter()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect();
        Solved::new(result)
    });
}
