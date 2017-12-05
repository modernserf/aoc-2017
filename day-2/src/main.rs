use std::cmp;

fn main() {
    let tsv = include_str!("input.txt");
    let data = parse_tsv(tsv);

    let sum = data.iter()
        .map(|row| row_checksum(row))
        .sum::<u32>();

    println!("part 1: {}", sum);

    let div_sum = data.iter()
        .map(|row| divisible_row_checksum(row))
        .sum::<u32>();

    println!("part 2: {}", div_sum);
}


fn parse_tsv(tsv: &str) -> Vec<Vec<u32>> {
    tsv.split("\n")
        .map(|row| {
            row.split("\t")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|row| { row.len() > 0 })
        .collect()
}


fn row_checksum(xs: &[u32]) -> u32 {
    let (min, max) = min_max(xs);
    max - min
}


fn divisible_row_checksum(xs: &[u32]) -> u32 {
    for (i, x) in xs.iter().enumerate() {
        let ys = &xs[(i+1)..];
        for y in ys {
            let (a, b) = order(*x, *y);
            if b % a == 0 {
                return b / a;
            }
        }
    }

    panic!("Couldn't find divisible items in row")
}


fn min_max<T: Ord + Copy>(xs: &[T]) -> (T, T) {
    xs.iter().fold(
        (xs[0], xs[0]),
        |(min_val, max_val), x| {
            (cmp::min(*x, min_val), cmp::max(*x, max_val))
        })
}


fn order<T: PartialOrd> (a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}
