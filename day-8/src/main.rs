#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");

    let mut memory = Memory::new();

    for line in contents.lines() {
        if line.len() > 0 {
            println!("line {}", line);
            let instr = parse_line(line);
            memory.run(&instr);
        }
    }

    let largest_value = memory.largest_value();
    println!("{}", largest_value);
}


#[derive(Debug)]
struct Memory {
    data: HashMap<String, i32>
}

impl Memory {
    fn new() -> Memory {
        Memory { data: HashMap::new() }
    }
    fn run(&mut self, instr: &Instruction) {
        let source_value = self.get(&instr.source);
        if comp(&instr.operator, source_value, instr.condition_value) {
            if let Some(h) = self.data.get_mut(&instr.target) {
                *h += instr.offset();
                return
            }
            self.data.insert(instr.target.to_string(), instr.offset());
        }
    }
    fn get(&self, id: &str) -> i32 {
        *self.data.get(id).unwrap_or(&0)
    }
    fn largest_value(&self) -> i32 {
        *self.data.values().max().unwrap()
    }
}


fn comp(op: &Op, l: i32, r: i32) -> bool {
    match op {
        &Op::Gt => { l > r },
        &Op::Gte => { l >= r },
        &Op::Eq => { l == r },
        &Op::Neq => { l != r },
        &Op::Lte => { l <= r },
        &Op::Lt => { l < r },
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Inc,
    Dec,
}

#[derive(Debug)]
enum Op {
    Gt,
    Gte,
    Eq,
    Neq,
    Lte,
    Lt,
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
        if self.direction == Direction::Inc {
            self.value
        } else {
            -self.value
        }
    }
}

fn parse_line(s: &str) -> Instruction {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\w+) (\w+) (-?\d+) if (\w+) ([=<>!]+) (-?\d+)").unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            let target = String::from(&cap[1]);

            let direction = if &cap[2] == "inc" {
                Direction::Inc
            } else {
                Direction::Dec
            };

            let value = cap[3].parse::<i32>().expect("source value");

            let source = String::from(&cap[4]);

            let operator = match &cap[5] {
                ">"  => Op::Gt,
                ">=" => Op::Gte,
                "==" => Op::Eq,
                "!=" => Op::Neq,
                "<=" => Op::Lte,
                "<"  => Op::Lt,
                _ => { panic!("unknown pattern"); },
            };

            let condition_value = cap[6].parse::<i32>().expect("condition value");

            Instruction {
                target,
                direction,
                value,
                source,
                operator,
                condition_value,
            }
        })
        .nth(0).unwrap()
}
