fn main() {
    let mut a = Gen::new(16807, 516, 1);
    let mut b = Gen::new(48271, 190, 1);

    let count = a.compare(&mut b, 40_000_000);
    println!("part 1: {}", count);

    let mut a = Gen::new(16807, 516, 4);
    let mut b = Gen::new(48271, 190, 8);

    let count = a.compare(&mut b, 5_000_000);
    println!("part 2: {}", count);
}

const GEN_DIV : u64 = 2147483647;
const BIT_MASK : u32 = 0xFFFF;

struct Gen {
    factor: u32,
    state: u32,
    modulus: u32,
}

impl Gen {
    fn new(factor: u32, state: u32, modulus: u32) -> Gen {
        Gen { factor, state, modulus }
    }
    fn next (&mut self) -> u32 {
        let next = (self.state as u64 * self.factor as u64) % GEN_DIV;
        let next = next as u32;
        self.state = next;

        if next % self.modulus == 0 {
            next
        } else {
            self.next()
        }
    }
    fn compare(&mut self, other: &mut Gen, iters: u32) -> u32 {
        let mut count = 0;
        for _ in 0..iters {
            let a_val = self.next() & BIT_MASK;
            let b_val = other.next() & BIT_MASK;
            if a_val == b_val { count += 1 }
        }
        count
    }
}
