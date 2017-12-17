fn main() {
    let mut rbuf = RingBuffer::new(380);
    rbuf.build(2017);
    println!("part 1: {}", rbuf.following_value());

    let mut rbuf = RingBuffer::new(380);
    rbuf.mock_build(50_000_000);
    println!("part 2: {}", rbuf.value_after_zero());
}

struct RingBuffer {
    data: Vec<u32>,
    position: usize,
    step_size: usize,
    len: usize,
    // part 2
    zero_position: usize,
    one_value: u32,
}

impl RingBuffer {
    fn new(step_size: usize) -> RingBuffer {
        RingBuffer {
            data: vec![0 as u32],
            position: 0,
            step_size,
            len: 1,
            zero_position: 0,
            one_value: 0,
        }
    }
    fn insert (&mut self, value: u32) {
        let next_position = (self.position + self.step_size + 1) % self.len;

        self.data.insert(next_position, value);
        self.position = next_position;
        self.len += 1;
    }
    fn mock_insert (&mut self, value: u32) {
        let next_position = (self.position + self.step_size + 1) % self.len;

        if next_position == ((self.zero_position + 1) % self.len) {
            self.one_value = value;
        }

        if next_position <= self.zero_position {
            self.zero_position += 1;
        }

        self.position = next_position;
        self.len += 1;
    }
    fn build (&mut self, count: u32) {
        for i in 0..count {
            self.insert(i + 1);
        }
    }
    fn mock_build(&mut self, count: u32) {
        for i in 0..count {
            self.mock_insert(i + 1);
        }
    }
    fn following_value(&self) -> u32 {
        let pos = (self.position + 1) % self.len;
        self.data[pos]
    }
    fn value_after_zero(&self) -> u32 {
        self.one_value
    }
}
