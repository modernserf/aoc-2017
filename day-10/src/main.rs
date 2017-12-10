fn main() {
    let input = "106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36";
    let data = input.split(",")
        .filter_map(|x| x.parse::<u8>().ok())
        .collect::<Vec<_>>();
    let mut ring = Ring::new(256);

    ring.run_twists(&data);
    println!("part 1: {}", ring.head_factor());

    let mut ring = Ring::new(256);
    ring.hash_str(&input);
    let dense_hash = ring.dense_hash();
    println!("part 2: {}", bytes_to_hex(&dense_hash));
}

struct Ring {
    data: Vec<u8>,
    suffix: Vec<u8>,
}

impl Ring {
    fn new (size: i32) -> Ring {
        let mut data = Vec::new();
        let suffix = vec![17, 31, 73, 47, 23];

        for i in 0..size {
            data.push(i as u8);
        }
        Ring { data, suffix }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get(&self, index: i32) -> u8 {
        self.data[index as usize % self.data.len()]
    }
    fn swap(&mut self, l: i32, r: i32) {
        let ln = self.len() as i32;
        let l = l % ln;
        let r = r % ln;
        self.data.swap(l as usize, r as usize);
    }
    fn twist(&mut self, index: i32, length: i32) {
        for i in 0..(length / 2) {
            let src_i = index + i;
            let dest_i = index + length - 1 - i;
            self.swap(src_i, dest_i);
        }
    }
    fn run_twists(&mut self, xs: &[u8]) {
        let mut index = 0;
        let mut skip = 0;
        for length in xs.iter() {
            let length = *length as i32;
            self.twist(index, length);
            index += length + skip;
            skip += 1;
        }
    }
    fn head_factor(&self) -> i32 {
        self.get(0) as i32 * self.get(1) as i32
    }
    // part 2
    fn hash_str(&mut self, s: &str) {
        let mut index = 0;
        let mut skip = 0;
        let mut items = String::from(s).into_bytes();
        items.extend(self.suffix.to_vec());

        for _round in 0..64 {
            for length in items.iter() {
                let length = *length as i32;
                self.twist(index, length);
                index += length + skip;
                skip += 1;
            }
        }


    }
    fn dense_hash(&self) -> Vec<u8> {
        let mut hash = Vec::new();
        for page in 0..self.len() / 16 {
            let mut acc: u8 = 0;
            for i in 0..16 {
                acc ^= self.get((page * 16 + i) as i32);
            }
            hash.push(acc);
        }
        hash
    }
}

fn bytes_to_hex (xs: &[u8]) -> String {
    let mut s = String::new();
    for x in xs.iter() {
        let op = format!("{:x}", x);
        if op.len() == 1 {
            s += &"0"
        }
        s += &op;
    }
    s
}
