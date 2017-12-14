extern crate knot_hash;
use knot_hash::Ring;
use std::collections::HashSet;

fn main() {
    let input = "wenycdww";
    let matrix = Matrix::new(&input);

    println!("part 1: {}", matrix.count());
    println!("part 2: {}", matrix.region_count());
}

struct Matrix {
    data: Vec<Vec<u8>>,
}

impl Matrix {
    fn new(input: &str) -> Matrix {
        let data = (0..128).map(|i| {
            let s = format!("{}-{}", input, i);
            let mut ring = Ring::new(256);
            ring.hash_str(&s);
            ring.dense_hash()
        }).collect::<Vec<Vec<u8>>>();

        Matrix { data }
    }
    fn count(&self) -> u32 {
        (0..128).fold(0, |sum, y| {
            sum + (0..128).fold(0, |sum_inner, x| {
                if self.get_bit(x, y) { sum_inner + 1 } else { sum_inner }
            })
        }) as u32
    }
    fn get_bit(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= 128 || y >= 128 { return false }
        let x = x as usize;
        let y = y as usize;
        let row = &self.data[y];
        let col = row[x >> 3];      // x / 8
        let bit_i = 128 >> (x & 7); // x % 8
        (col & bit_i) == bit_i
    }
    fn fill(&self, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>) {
        if visited.contains(&(x, y)) { return }
        visited.insert((x, y));

        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            if self.get_bit(x + dx, y + dy) {
                self.fill(x + dx, y + dy, visited);
            }
        }
    }
    fn region_count(&self) -> u32 {
        let mut region = 0;
        let mut visited = HashSet::new();
        for y in 0..128 {
            for x in 0..128 {
                if visited.contains(&(x, y)) { continue; }
                if self.get_bit(x, y) {
                    region += 1;
                    self.fill(x, y, &mut visited);
                }
            }
        }
        region
    }
}
