use std::collections::HashMap;

fn main() {
    let init = Pattern::from_string(".#./..#/###");

    let contents = include_str!("input.txt");
    let pattern_matcher = PatternMatcher::from_string(&contents);

    let result = (0..5).fold(init.copy(), |state, _| {
        pattern_matcher.grow(&state)
    });

    println!("part 1: {}", result.checksum());

    let result = (0..18).fold(init.copy(), |state, _| {
        pattern_matcher.grow(&state)
    });

    println!("part 2: {}", result.checksum());
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pattern {
    value: Vec<bool>,
    size: usize,
}

impl Pattern {
    fn new (size: usize) -> Pattern {
        Pattern { value: vec![false; size * size], size }
    }
    fn copy (&self) -> Pattern {
        Pattern {
            value: self.value.to_vec(),
            size: self.size,
        }
    }
    fn from_string (s: &str) -> Pattern {
        let mut value = Vec::new();
        let mut size = 0;

        for row in s.split("/") {
            size += 1;
            for ch in row.chars() {
                let x = match ch {
                    '.' => false,
                    '#' => true,
                    _ => { panic!("unknown char"); }
                };
                value.push(x);
            }
        }

        Pattern { value, size }
    }
    fn get (&self, x: usize, y: usize) -> bool {
        let i = (y * self.size) + x;
        self.value[i]
    }
    fn set (&mut self, x: usize, y: usize, value: bool) {
        let i = (y * self.size) + x;
        self.value[i] = value;
    }
    fn transform(&self, t: fn(usize, usize, usize) -> (usize, usize)) -> Pattern {
        let mut out = Pattern::new(self.size);

        for y in 0..self.size {
            for x in 0..self.size {
                let (tx, ty) = t(x, y, self.size - 1);
                out.set(x, y, self.get(tx, ty));
            }
        }

        out
    }
    fn permutations(&self) -> Vec<Pattern> {
        vec![
            self.copy(),
            self.transform(|x, y, s| (x, s - y)),
            self.transform(|x, y, s| (s - x, y)),
            self.transform(|x, y, s| (s - x, s - y)),
            self.transform(|x, y, _| (y, x)),
            self.transform(|x, y, s| (y, s - x)),
            self.transform(|x, y, s| (s - y, x)),
            self.transform(|x, y, s| (s - y, s - x)),
        ]
    }
    fn next_size(&self) -> usize {
        if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            panic!("dont know what to do with this size");
        }
    }
    fn split (&self) -> Vec<Pattern> {
        let next_size = self.next_size();
        let parts = self.size / next_size;
        let mut res = Vec::new();

        for i in 0..parts {
            for j in 0..parts {
                let mut p = Pattern::new(next_size);
                for y in 0..next_size {
                    for x in 0..next_size {
                        let val = self.get(
                            x + (next_size * j), y + (next_size * i));
                        p.set(x, y, val);
                    }
                }
                res.push(p)
            }
        }

        res
    }
    fn join (items: &[Pattern]) -> Pattern {
        let splits = sqrt(items.len());
        let inner_size = items[0].size;
        let join_size = splits * inner_size;
        let mut res = Pattern::new(join_size);
        for y in 0..join_size {
            for x in 0..join_size {
                let i = (x / inner_size) + (y / inner_size) * splits;
                let val = items[i].get(x % inner_size, y % inner_size);
                res.set(x, y, val);
            }
        }

        res
    }
    fn checksum (&self) -> usize {
        let mut sum = 0;
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get(x,y) {
                    sum += 1;
                }
            }
        }
        sum
    }
}

struct PatternMatcher {
    items: HashMap<Pattern, Pattern>,
}

impl PatternMatcher {
    fn from_string(s: &str) -> PatternMatcher {
        let mut items = HashMap::new();

        for line in s.lines() {
            if line.len() == 0 { continue }
            let parts = line.split(" => ")
                .map(|l| Pattern::from_string(l))
                .collect::<Vec<Pattern>>();

            let value = &parts[1];

            for key in parts[0].permutations().iter() {
                items.insert(key.copy(), value.copy());
            }
        }

        PatternMatcher { items }
    }
    fn get (&self, p: &Pattern) -> Pattern {
        self.items.get(p).unwrap().copy()
    }
    fn grow (&self, p: &Pattern) -> Pattern {
        if p.size <= 3 { return self.get(p); }

        let sub_ps = p.split().iter()
            .map(|sub_p| self.grow(sub_p))
            .collect::<Vec<Pattern>>();

        Pattern::join(&sub_ps)
    }
}

fn sqrt (x: usize) -> usize {
    for i in 0..1000 {
        if (i * i) == x { return i }
    }
    panic!("no integer sqrt in range {}", x);
}
