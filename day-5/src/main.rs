use std::fs::File;
use std::io::Read;

fn main() {
    let contents = get_file_contents();
    let mut buf = read_into_buffer(&contents);

    let steps = traverse_buffer(&mut buf);
    println!("step 1: {}", steps);

    let mut buf = read_into_buffer(&contents);
    let steps = traverse_buffer_decreasing(&mut buf);
    println!("step 2: {}", steps);
}

fn get_file_contents() -> String {
    let mut file = File::open("./src/input.txt")
        .expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file as string");

    contents
}

fn read_into_buffer(s: &str) -> Vec<i32> {
    s.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn traverse_buffer(buf: &mut Vec<i32>) -> u32 {
    let mut ptr: i32 = 0;
    for i in 0..1_000_000_000 {
        if let Some(val) = buf.get_mut(ptr as usize) {
            ptr += *val;
            *val += 1;
        } else {
            return i;
        }
    }

    panic!("could not complete in 1b iterations")
}

fn traverse_buffer_decreasing(buf: &mut Vec<i32>) -> u32 {
    let mut ptr: i32 = 0;
    for i in 0..1_000_000_000 {
        if let Some(val) = buf.get_mut(ptr as usize) {
            let offset = if *val >= 3 { -1 } else { 1 };
            ptr += *val;
            *val += offset;
        } else {
            return i;
        }
    }

    panic!("could not complete in 1b iterations")
}
