use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let program = contents.lines()
        .filter(|line| line.len() > 0)
        .map(|line| Inst::parse(line))
        .collect::<Vec<Inst>>();

    let mut vm = VM::new(program);
    let freq = vm.get_first_frequency();

    println!("part 1: {}", freq);
}

enum Value {
    Register(char),
    Constant(i32),
}

impl Value {
    fn parse (word: &str) -> Value {
        if let Ok(val) = word.parse::<i32>() {
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
    registers: HashMap<char, i32>,
    last_sound: i32,
    counter: usize,
    program: Vec<Inst>,
}

impl VM {
    fn new (program: Vec<Inst>) -> VM {
        VM {
            registers: HashMap::new(),
            last_sound: 0,
            counter: 0,
            program: program,
        }
    }
    fn set (&mut self, reg: char, value: i32) {
        self.registers.insert(reg, value);
    }
    fn get_register(&self, ch: char) -> i32 {
        self.registers.get(&ch).map(|c| *c).unwrap_or_else(|| 0)
    }
    fn get (&self, val: &Value) -> i32 {
        match val {
            &Value::Register(ch) => { self.get_register(ch)  },
            &Value::Constant(v) => { v },
        }
    }
    fn op (&mut self, ch: char, v: &Value, op: fn(i32, i32) -> i32) {
        let l = self.get_register(ch);
        let r = self.get(v);
        let result = op(l, r);
        self.set(ch, result);
    }
    fn get_instruction (&mut self, counter: usize) -> &Inst {
        self.program.get(counter).unwrap()
    }
    fn step (&mut self) -> bool {
        if self.program.get(self.counter).is_none() {
            return false
        }

        let counter = self.counter;
        let instruction = self.get_instruction(counter);

        match instruction {
            &Inst::Snd(ref v) => {
                let val = self.get(&v);
                self.last_sound = val;
            },
            &Inst::Set(ch, ref v) => {
                let val = self.get(&v);
                self.set(ch, val);
            },
            &Inst::Add(ch, ref v) => { self.op(ch, &v, |l, r| l + r); },
            &Inst::Mul(ch, ref v) => { self.op(ch, &v, |l, r| l * r); },
            &Inst::Mod(ch, ref v) => { self.op(ch, &v, |l, r| l % r); },
            &Inst::Rcv(ch) => {
                let val = self.get_register(ch);
                if val != 0 {
                    let last_sound = self.last_sound;
                    self.set(ch, last_sound);
                    return false;
                }
            },
            &Inst::Jgz(ref val, ref offset) => {
                let val = self.get(&val);
                let offset = self.get(&offset);
                if val > 0 {
                    let next_counter = (self.counter as i32) + offset;
                    self.counter = next_counter as usize;
                    return true; // return early to skip incrementing counter
                }
            }
        }
        self.counter += 1;
        return true
    }
    fn get_first_frequency (&mut self) -> i32 {
        loop {
            let next = self.step();
            if !next { break }
        }
        self.last_sound
    }
}
