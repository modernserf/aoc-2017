use std::collections::HashSet;


fn main() {
    let input = vec![14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];

    let (count, next_input) = find_redistribution_count(&input);
    println!("part 1: {}", count);

    let (count, _) = find_redistribution_count(&next_input);
    println!("part 2: {}", count);
}


fn find_redistribution_count(buf: &[usize]) -> (usize, Vec<usize>) {
    let mut seen_inputs = HashSet::new();
    let mut r = buf.to_vec();
    let ln = buf.len();

    for iters in 1..10_000_000 {
        seen_inputs.insert(r.clone());

        let (offset, max) = find_max_index_value(&r);

        r[offset] = 0;

        for j in 1..(max + 1) {
            r[(offset + j) % ln] += 1;
        }

        if seen_inputs.contains(&r) {
            return (iters, r.to_vec());
        }
    }

    panic!("no duplicates seen");
}


fn find_max_index_value(r: &[usize]) -> (usize, usize) {
    r.iter().enumerate()
        .fold((0, r[0]), |(max_i, max_val), (i, val)| {
            if *val > max_val { (i, *val) } else { (max_i, max_val) }
        })
}
