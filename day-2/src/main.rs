use std::fs::File;
use std::io::Read;

fn main() {
    let tsv = get_file_contents();
    let data = parse_tsv(tsv);

    let mut sum = 0;
    let mut div_sum = 0;
    for row in data {
        sum += row_checksum(&row);
        div_sum += divisible_row_checksum(&row);
    }

    println!("part 1: {}", sum);
    println!("part 2: {}", div_sum);
}


fn get_file_contents() -> String {
    let mut file = File::open("./src/input.txt")
        .expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file as string");

    contents
}

fn parse_tsv(tsv: String) -> Vec<Vec<u32>> {
    let mut data = Vec::new();
    let rows = tsv.split("\n");

    for row in rows {
        let mut row_data = Vec::new();
        let cols = row.split("\t");

        for col in cols {
            if let Ok(val) = col.parse::<u32>() {
                row_data.push(val);
            }
        }

        if row_data.len() > 0 {
            data.push(row_data);
        }
    }

    data
}

fn row_checksum(xs: &Vec<u32>) -> u32 {
    let (min, max) = minmax(xs);
    max - min
}

fn divisible_row_checksum(xs: &Vec<u32>) -> u32 {
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

fn minmax<T: PartialOrd + Copy>(xs: &[T]) -> (T, T) {
    let mut min = xs[0];
    let mut max = xs[0];

    for x in xs {
        if *x > max { max = *x; }
        if *x < min { min = *x; }
    }

    (min, max)
}

fn order<T: PartialOrd> (a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}
