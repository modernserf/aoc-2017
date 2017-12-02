use std::fs::File;
use std::io::Read;

fn main() {
    let contents = get_file_contents();
    let digits = string_to_numbers(contents);
    // part 1
    let sum = sum_matching_neighbors(&digits);
    println!("part 1: {}", sum);

    // part 2
    let sum = sum_matching_opposites(&digits);
    println!("part 2: {}", sum);
}

fn get_file_contents() -> String {
    let mut file = File::open("./src/input.txt")
        .expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file as string");

    contents
}

fn string_to_numbers(s: String) -> Vec<u32> {
    let mut vec = Vec::new();

    for ch in s.split("") {
        if let Ok(num) = ch.parse::<u32>() {
            vec.push(num);
        }
    }

    vec
}

fn match_value<T: PartialEq> (l: T, r: T) -> Option<T> {
    if l == r {
        Some(l)
    } else {
        None
    }
}


fn sum_at_offset (vec: &[u32], offset: usize) -> u32 {
    let mut sum = 0;
    let ln = vec.len();

    for i in 0..ln {
        let l = vec[i];
        let r = vec[(i + offset) % ln];
        if let Some(val) = match_value(l, r) {
            sum = sum + val
        }
    }

    sum
}


fn sum_matching_neighbors (vec: &[u32]) -> u32 {
    sum_at_offset(vec, 1)
}

fn sum_matching_opposites (vec: &[u32]) -> u32 {
    let offset = vec.len() / 2;
    sum_at_offset(vec, offset)
}
