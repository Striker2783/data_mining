/// A nested loop function that loops through the data, sending an array of k size to the function
pub fn nested_loops<T: FnMut(&[usize])>(mut f: T, data: &[usize], k: usize) {
    let mut stack = vec![0; k];
    nested_loops_helper(&mut f, data, 0, 0, k, &mut stack);
}
/// The recursive helper for nested_loops
fn nested_loops_helper<T: FnMut(&[usize])>(
    f: &mut T,
    data: &[usize],
    i: usize,
    start: usize,
    k: usize,
    stack: &mut [usize],
) {
    // If the number of elements on the stack is equal to k, call f
    if i == k {
        f(stack);
        return;
    }
    // Loop through the rest of the data, add it to the stack, and recursively call
    for j in start..data.len() {
        stack[i] = data[j];
        nested_loops_helper(f, data, i + 1, j + 1, k, stack);
    }
}
/// Same as nested_loops, but with a prune function
pub fn nested_loops_prune<T: FnMut(&[usize]), U: Fn(&[usize]) -> bool>(
    mut f: T,
    prune: U,
    data: &[usize],
    k: usize,
) {
    let mut stack = vec![0; k];
    nested_loops_prune_helper(&mut f, &prune, data, 0, 0, k, &mut stack);
}
fn nested_loops_prune_helper<T: FnMut(&[usize]), U: Fn(&[usize]) -> bool>(
    f: &mut T,
    p: &U,
    data: &[usize],
    i: usize,
    start: usize,
    k: usize,
    stack: &mut [usize],
) {
    if i == k {
        f(stack);
        return;
    }
    if p(&stack[..i]) {
        return;
    }
    for j in start..data.len() {
        stack[i] = data[j];
        nested_loops_prune_helper(f, p, data, i + 1, j + 1, k, stack);
    }
}
