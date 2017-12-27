use std::collections::HashMap;


fn main() {
    // state, value, write, next_tape, next_state
    let mut machine = TuringMachine::new('a', &vec![
        ('a', 0, 1, 1, 'b'),
        ('a', 1, 0, -1, 'c'),

        ('b', 0, 1, -1, 'a'),
        ('b', 1, 1, 1, 'c'),

        ('c', 0, 1, 1, 'a'),
        ('c', 1, 0, -1, 'd'),

        ('d', 0, 1, -1, 'e'),
        ('d', 1, 1, -1, 'c'),

        ('e', 0, 1, 1, 'f'),
        ('e', 1, 1, 1, 'a'),

        ('f', 0, 1, 1, 'a'),
        ('f', 1, 1, 1, 'e'),
    ]);

    for i in 0..12_134_527 {
        machine.step();
    }

    println!("part 1: {}", machine.checksum());
}

#[derive(Debug, Copy, Clone)]
struct StateTransition {
    write: bool,
    next_tape: i32,
    next_state: char,
}

struct TuringMachine {
    tape: HashMap<i32, bool>,
    tape_position: i32,
    state_map: HashMap<(char, bool), StateTransition>,
    state: char,
}

impl TuringMachine {
    fn new(init_state: char, instrs: &[(char, i32, i32, i32, char)]) -> TuringMachine {
        let mut state_map = HashMap::new();
        for &(state, value, write, next_tape, next_state) in instrs.iter() {
            let value = value == 1;
            let write = write == 1;
            state_map.insert(
                (state, value),
                StateTransition { write, next_tape, next_state }
            );
        }

        TuringMachine {
            tape: HashMap::new(),
            tape_position: 0,
            state: init_state,
            state_map,
        }
    }
    fn get_tape(&self) -> bool {
        self.tape.get(&self.tape_position).map(|x| *x).unwrap_or_else(|| false)
    }
    fn get_transition(&self) -> StateTransition {
        self.state_map.get(&(self.state, self.get_tape())).map(|x| *x).unwrap()
    }
    fn step (&mut self) {
        let txs = self.get_transition();
        self.tape.insert(self.tape_position, txs.write);
        self.tape_position += txs.next_tape;
        self.state = txs.next_state;
    }
    fn checksum (&self) -> usize {
        self.tape.values().fold(0, |sum, val| {
            if *val { sum + 1 } else { sum }
        })
    }
}
