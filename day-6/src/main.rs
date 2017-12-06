use std::collections::HashSet;


fn main() {
    let input = vec![14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];

    let (count, next_input) = find_redistribution_count(&input);
    println!("part 1: {}", count);

    let (count, _) = find_redistribution_count(&next_input);
    println!("part 2: {}", count);
}


fn find_redistribution_count(buf: &[u8]) -> (usize, Vec<u8>) {
    let mut seen_inputs = HashSet::new();
    let mut r = buf.to_vec();

    loop {
        seen_inputs.insert(r.clone());
        redistribute(&mut r);

        if seen_inputs.contains(&r) {
            return (seen_inputs.len(), r.to_vec());
        }
    }
}


fn find_max_index_value(r: &[u8]) -> (usize, u8) {
    r.iter().enumerate()
        .fold((0, r[0]), |(max_i, max_val), (i, val)| {
            if *val > max_val { (i, *val) } else { (max_i, max_val) }
        })
}


fn redistribute(r: &mut [u8]) {
    let ln = r.len();
    let (offset, max) = find_max_index_value(&r);

    r[offset] = 0;

    for j in 1..(max as usize + 1) {
        r[(offset + j) % ln] += 1;
    }
}
