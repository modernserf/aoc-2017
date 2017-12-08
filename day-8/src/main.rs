#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");

    let mut memory = Memory::new();

    for line in contents.lines() {
        if line.len() > 0 {
            let instr = parse_line(line);
            memory.run(&instr);
        }
    }

    let largest_value = memory.largest_value();
    println!("part 1: {}", largest_value);

    println!("part 2: {}", memory.max_value);
}


#[derive(Debug)]
struct Memory {
    data: HashMap<String, i32>,
    max_value: i32
}

impl Memory {
    fn new() -> Memory {
        Memory { data: HashMap::new(), max_value: 0 }
    }
    fn run(&mut self, instr: &Instruction) {
        let source_value = self.get(&instr.source);
        if instr.operator.comp(source_value, instr.condition_value) {
            let next_value = self.get(&instr.target) + instr.offset();
            self.insert(&instr.target, next_value);
        }
    }
    fn get(&self, id: &str) -> i32 {
        *self.data.get(id).unwrap_or(&0)
    }
    fn largest_value(&self) -> i32 {
        *self.data.values().max().unwrap()
    }
    fn insert(&mut self, id: &str, value: i32) {
        self.data.insert(id.to_string(), value);
        if value > self.max_value {
            self.max_value = value;
        }
    }
}


#[derive(Debug)]
enum Direction { Inc, Dec }

impl Direction {
    fn from_string(s: &str) -> Option<Direction> {
        match s {
            "inc" => Some(Direction::Inc),
            "dec" => Some(Direction::Dec),
            _ => None,
        }
    }
    fn offset(&self, val: i32) -> i32 {
        match self {
            &Direction::Inc => val,
            &Direction::Dec => -val,
        }
    }
 }


#[derive(Debug)]
enum Op { Gt, Gte, Eq, Neq, Lte, Lt }

impl Op {
    fn from_string(s: &str) -> Option<Op> {
        match s {
            ">"  => Some(Op::Gt),
            ">=" => Some(Op::Gte),
            "==" => Some(Op::Eq),
            "!=" => Some(Op::Neq),
            "<=" => Some(Op::Lte),
            "<"  => Some(Op::Lt),
            _ => None
        }
    }
    fn comp(&self, l: i32, r: i32) -> bool {
        match self {
            &Op::Gt => { l > r },
            &Op::Gte => { l >= r },
            &Op::Eq => { l == r },
            &Op::Neq => { l != r },
            &Op::Lte => { l <= r },
            &Op::Lt => { l < r },
        }
    }
}


#[derive(Debug)]
struct Instruction {
    target: String,
    direction: Direction,
    value: i32,
    source: String,
    operator: Op,
    condition_value: i32,
}

impl Instruction {
    fn offset(&self) -> i32 {
        self.direction.offset(self.value)
    }
}

fn parse_line(s: &str) -> Instruction {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\w+) (\w+) (-?\d+) if (\w+) ([=<>!]+) (-?\d+)").unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            Instruction {
                target: String::from(&cap[1]),
                direction: Direction::from_string(&cap[2]).unwrap(),
                value: cap[3].parse::<i32>().expect("source value"),
                source: String::from(&cap[4]),
                operator: Op::from_string(&cap[5]).unwrap(),
                condition_value: cap[6].parse::<i32>().expect("condition value"),
            }
        })
        .nth(0).unwrap()
}
