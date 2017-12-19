use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let contents = include_str!("input.txt");
    let program = contents.lines()
        .filter(|line| line.len() > 0)
        .map(|line| Inst::parse(line))
        .collect::<Vec<Inst>>();

    let mut vm = VM::new(0);
    let freq = vm.get_first_frequency(&program);
    println!("part 1: {}", freq);

    let send_count = run_processes(&program);
    println!("part 2: {}", send_count);
}


enum Value {
    Register(char),
    Constant(i64),
}

impl Value {
    fn parse (word: &str) -> Value {
        if let Ok(val) = word.parse::<i64>() {
            Value::Constant(val)
        } else {
            let ch = parse_char(&word);
            Value::Register(ch)
        }
    }
}

fn parse_char (s: &str) -> char {
    s.chars().next().unwrap()
}

enum Inst {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

impl Inst {
    fn parse (line: &str) -> Inst {
        let parts = line.split(" ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        match parts[0].as_ref() {
            "snd" => { Inst::Snd(Value::parse(&parts[1])) },
            "set" => { Inst::Set(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "add" => { Inst::Add(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "mul" => { Inst::Mul(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "mod" => { Inst::Mod(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "rcv" => { Inst::Rcv(parse_char(&parts[1])) },
            "jgz" => { Inst::Jgz(Value::parse(&parts[1]), Value::parse(&parts[2])) },
            _     => { panic!("unknown instruction") },
        }
    }
}

struct VM {
    registers: HashMap<char, i64>,
    send_count: usize,
}

impl VM {
    fn new (pid: i64) -> VM {
        let mut registers = HashMap::new();
        registers.insert('p', pid);

        VM { registers, send_count: 0 }
    }
    fn set (&mut self, reg: char, value: i64) {
        self.registers.insert(reg, value);
    }
    fn get_register(&self, ch: char) -> i64 {
        self.registers.get(&ch).map(|c| *c).unwrap_or_else(|| 0)
    }
    fn get (&self, val: &Value) -> i64 {
        match val {
            &Value::Register(ch) => { self.get_register(ch)  },
            &Value::Constant(v) => { v },
        }
    }
    fn op (&mut self, ch: char, v: &Value, op: fn(i64, i64) -> i64) {
        let l = self.get_register(ch);
        let r = self.get(v);
        let result = op(l, r);
        self.set(ch, result);
    }
    fn step (&mut self, instruction: &Inst, ins: &mut VecDeque<i64>, outs: &mut VecDeque<i64>) -> i64 {
        match instruction {
            &Inst::Snd(ref v) => {
                let val = self.get(&v);
                self.send_count += 1;
                outs.push_back(val);
            },
            &Inst::Set(ch, ref v) => {
                let val = self.get(&v);
                self.set(ch, val);
            },
            &Inst::Add(ch, ref v) => { self.op(ch, &v, |l, r| l + r); },
            &Inst::Mul(ch, ref v) => { self.op(ch, &v, |l, r| l * r); },
            &Inst::Mod(ch, ref v) => { self.op(ch, &v, |l, r| l % r); },
            &Inst::Rcv(ch) => {
                if ins.len() > 0 {
                    let msg = ins.pop_front().unwrap();
                    self.set(ch, msg);
                } else {
                    return 0; // return early to skip incrementing counter
                }
            },
            &Inst::Jgz(ref val, ref offset) => {
                let val = self.get(&val);
                let offset = self.get(&offset);
                if val > 0 {
                    return offset; // return early to skip incrementing counter
                }
            }
        }
        1
    }
    fn get_first_frequency (&mut self, instructions: &[Inst]) -> i64 {
        let mut counter : i64 = 0;
        let mut outs = VecDeque::new();
        let mut ins = VecDeque::new();
        loop {
            if let Some(inst) = instructions.get(counter as usize) {
                let offset = self.step(inst, &mut ins, &mut outs);
                if offset != 0 {
                    counter += offset;
                    continue;
                }
            }
            return outs.pop_back().unwrap();
        }
    }
}

fn run_processes (instructions: &[Inst]) -> usize {
    let mut p_0 = VM::new(0);
    let mut p_1 = VM::new(1);
    let mut ch_0_to_1 = VecDeque::new();
    let mut ch_1_to_0 = VecDeque::new();
    let mut pc_0 : i64 = 0;
    let mut pc_1 : i64 = 0;
    loop {
        let offset_0 = p_0.step(&instructions[pc_0 as usize], &mut ch_1_to_0, &mut ch_0_to_1);
        let offset_1 = p_1.step(&instructions[pc_1 as usize], &mut ch_0_to_1, &mut ch_1_to_0);
        if offset_0 == 0 && offset_1 == 0 { break }

        pc_0 += offset_0;
        pc_1 += offset_1;
    }
    p_1.send_count
}
