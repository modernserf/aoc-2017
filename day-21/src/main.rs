fn main() {
    let init = Pattern::from_string(".#./..#/###");

    let contents = include_str!("input.txt");
    let pattern_matches = contents.lines()
        .filter(|line| line.len() > 0)
        .map(|line| PatternMatch::from_string(line))
        .collect::<Vec<PatternMatch>>();


    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
struct Pattern {
    value: Vec<Vec<bool>>,
}

impl Pattern {
    fn from_string (s: &str) -> Pattern {
        let value = s.split("/")
            .map(|row| {
                row.chars().map(| ch| {
                    match ch {
                        '.' => false,
                        '#' => true,
                        _ => { panic!("unknown char"); }
                    }
                })
                .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();

        Pattern { value }
    }
    fn to_int(&self) -> usize {
        let size = self.size();
        self.value.iter().enumerate().fold(0, |sum, (i, row)| {
            sum + row.iter().enumerate().fold(0, |sum_, (j, cell) {
                sum_ + if cell { 1 << ((i * size) + j) } else { 0 }
            })
        })
    }
    fn merge(ps:&[Pattern]) -> Pattern {

    }
    fn split(&self) -> Vec<Pattern> {
        if self.size() == 4 {
            let v = self.value;
            vec![
                Pattern { value:[[v[0][0], v[0][1]], [v[1][0], v[1][1]]] },
                Pattern { value:[[v[2][0], v[2][1]], [v[3][0], v[3][1]]] },
                Pattern { value:[[v[0][2], v[0][3]], [v[1][2], v[1][3]]] },
                Pattern { value:[[v[2][2], v[2][3]], [v[3][1], v[3][3]]] },
            ])
        } else {
            vec![*self]
        }
    }
    fn size(&self) -> usize {
        self.value.len()
    }
}

struct PatternMatch {
    from: Pattern,
    to: Pattern,
}

impl PatternMatch {
    fn from_string (s: &str) -> PatternMatch {
        let parts = s.split(" => ")
            .map(|p| Pattern::from_string(p))
            .collect::<Vec<Pattern>>();

        PatternMatch { from: parts[0], to: parts[1] }
    }
    fn grow (&self, p: &Pattern) -> Option<Pattern> {
        if p.matches(self.from) {
            Some(self.to)
        } else {
            None
        }
    }
}

fn grow(p: &Pattern, ms: &[PatternMatch]) -> Pattern {
    let sub_ps = p.split().iter().map(|sub_p| {
        for m of ms.iter() {
            if let Some(next_p) = m.grow(&p) {
                return next_p;
            }
        }
        panic!("no matching pattern found");
    });
    Pattern::merge(sub_ps)
}
