fn main() {
    let contents = include_str!("input.txt");
    let mut buf = read_into_buffer(contents);

    let steps = traverse_buffer(&mut buf);
    println!("step 1: {}", steps);

    let mut buf = read_into_buffer(contents);
    let steps = traverse_buffer_decreasing(&mut buf);
    println!("step 2: {}", steps);
}


fn read_into_buffer(s: &str) -> Vec<i32> {
    s.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}


fn traverse_buffer(buf: &mut Vec<i32>) -> u32 {
    let mut ptr: i32 = 0;
    for i in 0..1_000_000_000 {
        // N.B. this would false-positive if MAX_INT < buffer.len() < MAX_UINT
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
