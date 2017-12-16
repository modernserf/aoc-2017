use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let mut dance = Dance::new(16);

    let dance_moves = contents.split(",")
        .map(|s| { Instruction::parse(s) })
        .collect::<Vec<Instruction>>();

    dance.run(&dance_moves);
    let key = dance.to_string().unwrap();

    println!("part 1: {}", key);

    let mut dance = Dance::new(16);
    let (from, to) = find_loop(&mut dance, &dance_moves);
    let remainder = (1_000_000_000 - from) % (to - from);

    for _ in 1 .. remainder {
        dance.run(&dance_moves);
    }

    println!("part 2: {}", dance.to_string().unwrap());
}

struct Dance {
    members: Vec<u8>,
}

impl Dance {
    fn new (size: usize) -> Dance {
        let mut members = Vec::with_capacity(size);
        for i in 0..size {
            members.push(i as u8);
        }

        Dance { members }
    }
    fn spin (&mut self, count: usize) {
        for _ in 0..count {
            self.members.pop().map(|last| {
                self.members.insert(0, last);
            });
        }
    }
    fn exchange(&mut self, from: usize, to: usize) {
        self.members.swap(from, to);
    }
    fn partner(&mut self, l: char, r: char) {
        let lpos = self.members.iter().position(|x| { *x == char_value(l) });
        let rpos = self.members.iter().position(|x| { *x == char_value(r) });

        lpos.map(|from| {
            rpos.map(|to| {
                self.members.swap(from, to);
            });
        });
    }
    fn to_string(&self) -> Option<String> {
        let ascii = self.members.iter()
            .map(|x| { x + 97 }) // ascii offset
            .collect::<Vec<u8>>();

        String::from_utf8(ascii).ok()
    }
    fn run(&mut self, xs: &[Instruction]) {
        for x in xs.iter() {
            match x {
                &Instruction::Spin(count) => self.spin(count),
                &Instruction::Exchange(from, to) => self.exchange(from, to),
                &Instruction::Partner(l, r) => self.partner(l, r),
            }
        }
    }
}

fn char_value (c: char) -> u8 {
    c.to_digit(36).unwrap() as u8 - 10
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let rest = s.get(1..).unwrap();
        match s.chars().next().unwrap() {
            's' => {
                let count = rest.parse::<usize>().unwrap();
                Instruction::Spin(count)
            },
            'x' => {
                let splits = rest.split("/").map(|x| String::from(x)).collect::<Vec<String>>();
                let from = splits[0].parse::<usize>().unwrap();
                let to = splits[1].parse::<usize>().unwrap();
                Instruction::Exchange(from, to)
            },
            'p' => {
                let mut chars = rest.chars();
                let l = chars.next().unwrap();
                chars.next(); // skip slash
                let r = chars.next().unwrap();
                Instruction::Partner(l, r)
            },
            _ => { panic!("idk man") },
        }
    }
}

fn find_loop (dance: &mut Dance, dance_moves: &[Instruction]) -> (usize, usize) {
    let mut positions = HashMap::new();

    for i in 0..1_000_000 {
        dance.run(&dance_moves);
        let key = dance.to_string().unwrap();
        if let Some(val) = positions.get(&key) {
            return (*val, i);
        }
        positions.insert(key, i);
    }
    panic!("could not find loop in 1m iterations");
}
