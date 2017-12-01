use std::fs::File;
use std::io::Read;

fn main() {
    let contents = get_file_contents();
    let digits = string_to_numbers(contents);
    // part 1
    let sum = sum_matching_neighbors(&digits);
    println!("{}", sum);

    // part 2
    let sum = sum_matching_opposites(&digits);
    println!("{}", sum);
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
        match ch.parse::<u32>() {
            Ok(num) => { vec.push(num); },
            Err(_)  => { continue },
        }
    }

    vec
}

fn match_value (l: u32, r: u32) -> u32 {
    if l == r {
        l
    } else {
        0
    }
}


fn sum_at_offset (vec: &Vec<u32>, offset: usize) -> u32 {
    let mut sum = 0;

    let ln = vec.len();

    for i in 0..ln {
        let l = vec[i];
        let r = vec[(i + offset) % ln];
        sum = sum + match_value(l, r);
    }

    sum
}


fn sum_matching_neighbors (vec: &Vec<u32>) -> u32 {
    sum_at_offset(vec, 1)
}

fn sum_matching_opposites (vec: &Vec<u32>) -> u32 {
    let offset = vec.len() / 2;
    sum_at_offset(vec, offset)
}
