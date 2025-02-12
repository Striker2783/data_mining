pub fn nested_loops<T: FnMut(Vec<usize>)>(mut f: T, data: &[usize], k: usize) {
    let mut stack = vec![0; k];
    nested_loops_helper(&mut f, data, 0, k, &mut stack);
}
fn nested_loops_helper<T: FnMut(Vec<usize>)>(
    f: &mut T,
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
        f(v);
        return;
    }
    let start = if i == 0 { 0 } else { stack[i - 1] + 1 };
    for j in start..data.len() {
        stack[i] = j;
        nested_loops_helper(f, data, i + 1, k, stack);
    }
}
