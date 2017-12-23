use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let instructions = contents.lines()
        .filter(|line| line.len() > 0)
        .map(|line| Inst::parse(line))
        .collect::<Vec<Inst>>();

    let mul_count = run_mul_count(&instructions);
    println!("part 1: {}", mul_count);

    println!("part 2: {}", run_register_h(&instructions));
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
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

impl Inst {
    fn parse (line: &str) -> Inst {
        let parts = line.split(" ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        match parts[0].as_ref() {
            "set" => { Inst::Set(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "sub" => { Inst::Sub(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "mul" => { Inst::Mul(parse_char(&parts[1]), Value::parse(&parts[2])) },
            "jnz" => { Inst::Jnz(Value::parse(&parts[1]), Value::parse(&parts[2])) },
            _     => { panic!("unknown instruction") },
        }
    }
}

struct VM {
    registers: HashMap<char, i64>,
    mul_count: usize,
}

impl VM {
    fn new () -> VM {
        let registers = HashMap::new();

        VM { registers, mul_count: 0 }
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
    fn step (&mut self, instruction: &Inst) -> i64 {
        match instruction {
            &Inst::Set(ch, ref v) => {
                let val = self.get(&v);
                self.set(ch, val);
            },
            &Inst::Sub(ch, ref v) => {
                self.op(ch, &v, |l, r| l - r);
            },
            &Inst::Mul(ch, ref v) => {
                self.op(ch, &v, |l, r| l * r);
                self.mul_count += 1;
            },
            &Inst::Jnz(ref val, ref offset) => {
                let val = self.get(&val);
                let offset = self.get(&offset);
                if val != 0 {
                    return offset; // return early to skip incrementing counter
                }
            }
        }
        1
    }
}

fn run_mul_count (insts: &[Inst]) -> usize {
    let mut vm = VM::new();
    let mut pc = 0;

    loop {
        match insts.get(pc) {
            Some(inst) => {
                let offset = vm.step(inst);
                pc = ((pc as i64) + offset) as usize;
            },
            None => { break; }
        }
    }

    vm.mul_count
}


fn run_register_h (insts: &[Inst]) -> i64 {
    let mut vm = VM::new();
    // set non-debug mode
    vm.set('a', 1);
    let mut pc = 0;

    loop {
        match insts.get(pc) {
            Some(inst) => {
                let offset = vm.step(inst);
                pc = ((pc as i64) + offset) as usize;
            },
            None => { break; }
        }
    }

    vm.get_register('h')
}
