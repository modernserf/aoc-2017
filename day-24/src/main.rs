use std::cmp::Ordering;

fn main() {
    let content = include_str!("input.txt");
    let segments = content.lines()
        .filter(|l| l.len() > 0)
        .map(|l| BridgeSegment::from_string(l))
        .collect::<Vec<BridgeSegment>>();

    let bridges = Bridge::generate(&segments);
    let max_strength = bridges.iter().fold(0, |max, b| {
        b.strength().max(max)
    });
    println!("part 1: {}", max_strength);

    let (longest_strength, _) = bridges.iter().fold((0,0), |(max_str, max_len), b| {
        match b.len().cmp(&max_len) {
            Ordering::Less => { (max_str, max_len) },
            Ordering::Greater => { (b.strength(), b.len()) },
            Ordering::Equal => { (b.strength().max(max_str), max_len) },
        }
    });
    println!("part 2: {}", longest_strength);
}


#[derive(Debug, Copy, Clone)]
struct BridgeSegment {
    l: usize,
    r: usize,
}



impl BridgeSegment {
    fn from_string(s: &str) -> BridgeSegment {
        let parts = s.split("/")
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        BridgeSegment { l: parts[0], r: parts[1] }
    }
    fn strength(&self) -> usize {
        self.l + self.r
    }
    fn dir_value(&self, d: Direction) -> usize {
        match d {
            Direction::LR => self.r,
            Direction::RL => self.l,
        }
    }
    fn matches(&self, value: usize) -> Option<Direction> {
        if self.l == value {
            Some(Direction::LR)
        } else if self.r == value {
            Some(Direction::RL)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction { LR, RL }


struct Bridge {
    items: Vec<(BridgeSegment, Direction)>,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge { items: Vec::new() }
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn strength(&self) -> usize {
        self.items.iter().fold(0, |sum, &(segment, _)| sum + segment.strength())
    }
    fn tail_value(&self) -> usize {
        match self.items.last() {
            Some(&(segment, direction)) => segment.dir_value(direction),
            None => 0,
        }
    }
    fn append(&self, sg: BridgeSegment, dir: Direction) -> Bridge {
        let mut next_items = self.items.to_vec();
        next_items.push((sg, dir));

        Bridge { items: next_items }
    }
    fn try_push(&self, sg: BridgeSegment) -> Option<Bridge> {
        sg.matches(self.tail_value()).map(|direction| { self.append(sg, direction) })
    }
    fn generate_inner(&self, ins: &[BridgeSegment], index: usize, outs: &mut Vec<Bridge>) {
        if let Some(sg) = ins.get(index) {
            if let Some(next_bridge) = self.try_push(*sg) {
                let mut next_ins = ins.to_vec();
                next_ins.swap_remove(index);
                next_bridge.generate_inner(&next_ins, 0, outs);
                outs.push(next_bridge);
            }

            self.generate_inner(ins, index + 1, outs);
        }
    }
    fn generate(sgs: &[BridgeSegment]) -> Vec<Bridge> {
        let mut res = Vec::new();
        let init = Bridge::new();
        init.generate_inner(sgs, 0, &mut res);
        res
    }
}
