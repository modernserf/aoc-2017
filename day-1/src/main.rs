fn main() {
    let contents = include_str!("input.txt");
    let digits = string_to_numbers(contents);
    // part 1
    let sum = sum_matching_neighbors(&digits);
    println!("part 1: {}", sum);

    // part 2
    let sum = sum_matching_opposites(&digits);
    println!("part 2: {}", sum);
}


fn string_to_numbers(s: &str) -> Vec<u32> {
    s.split("")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}


fn match_value<T: PartialEq> (l: T, r: T) -> Option<T> {
    if l == r { Some(l) } else { None }
}


fn sum_at_offset (xs: &[u32], offset: usize) -> u32 {
    let ln = xs.len();

    xs.iter().enumerate().fold(0, |sum, (i, x)| {
        let y = &xs[(i + offset) % ln];
        match_value(x, y)
            .map_or(sum, |val| sum + val)
    })
}


fn sum_matching_neighbors (vec: &[u32]) -> u32 {
    sum_at_offset(vec, 1)
}


fn sum_matching_opposites (vec: &[u32]) -> u32 {
    let offset = vec.len() / 2;
    sum_at_offset(vec, offset)
}
