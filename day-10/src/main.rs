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
        let suffix = vec![17, 31, 73, 47, 23];
        let data = (0..size).map(|i| i as u8).collect::<Vec<u8>>();
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
    fn hash_rounds(&mut self, rounds: usize, xs: &[u8]) {
        let mut index = 0;
        let mut skip = 0;

        for _round in 0..rounds {
            for length in xs.iter() {
                let length = *length as i32;
                self.twist(index, length);
                index += length + skip;
                skip += 1;
            }
        }
    }
    fn run_twists(&mut self, xs: &[u8]) {
        self.hash_rounds(1, xs);
    }
    fn head_factor(&self) -> i32 {
        self.get(0) as i32 * self.get(1) as i32
    }
    // part 2
    fn hash_str(&mut self, s: &str) {
        let mut xs = String::from(s).into_bytes();
        xs.extend(self.suffix.to_vec());
        self.hash_rounds(64, &xs);
    }
    fn dense_hash(&self) -> Vec<u8> {
        let page_size = 16;
        (0..&self.len() / page_size).map(|page| {
            (0..page_size).fold(0, |acc, i| {
                acc ^ self.get((page * page_size + i) as i32)
            })
        }).collect::<Vec<u8>>()
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
