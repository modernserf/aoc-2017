fn main() {
    let contents = include_str!("input.txt");
    let plane = Plane::from_string(&contents);
    let cursor = plane.traverse();
    println!("part 1: {}", cursor.visited_str());
    println!("part 2: {}", cursor.steps);
}

#[derive(Debug)]
enum Path {
    Empty,
    NS,
    EW,
    Inter,
    Letter(char),
}

impl Path {
    fn parse(ch: char) -> Path {
        match ch {
            ' ' => Path::Empty,
            '|' => Path::NS,
            '-' => Path::EW,
            '+' => Path::Inter,
            _   => Path::Letter(ch),
        }
    }
}

#[derive(Debug)]
enum Dir { N, E, S, W }

impl Dir {
    fn left (&self) -> Dir {
        match self {
            &Dir::N => Dir::W,
            &Dir::W => Dir::S,
            &Dir::S => Dir::E,
            &Dir::E => Dir::N,
        }
    }
    fn right (&self) -> Dir {
        match self {
            &Dir::N => Dir::E,
            &Dir::E => Dir::S,
            &Dir::S => Dir::W,
            &Dir::W => Dir::N,
        }
    }
}

struct Cursor {
    x: usize,
    y: usize,
    direction: Dir,
    visited: Vec<char>,
    steps: usize,
}

impl Cursor {
    fn new(x: usize, y: usize) -> Cursor {
        Cursor { x, y, direction: Dir::S, visited: Vec::new(), steps: 0 }
    }
    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn visit(&mut self, ch: char) {
        self.visited.push(ch);
    }
    fn next_pos(&self, direction: &Dir) -> (usize, usize) {
        let (dx, dy) = match direction {
            &Dir::N => { (0, -1) },
            &Dir::E => { (1, 0) },
            &Dir::S => { (0, 1) },
            &Dir::W => { (-1, 0) },
        };
        let x = (self.x as i32) + dx;
        let y = (self.y as i32) + dy;
        (x as usize, y as usize)
    }
    fn step(&mut self) {
        let (x, y) = self.next_pos(&self.direction);
        self.x = x;
        self.y = y;
        self.steps += 1;
    }
    fn visited_str(&self) -> String {
        self.visited.iter().cloned().collect()
    }
    fn turn(&mut self, dir: Dir) {
        self.direction = dir;
    }
}

struct Plane {
    data: Vec<Vec<Path>>,
}

impl Plane {
    fn from_string(s: &str) -> Plane {
        let data = s.lines()
            .map(|l| {
                l.chars()
                    .map(|ch| Path::parse(ch))
                    .collect::<Vec<Path>>()
            })
            .collect::<Vec<Vec<Path>>>();

        Plane { data }
    }
    fn find_start_point (&self) -> (usize, usize) {
        for (x, p) in self.data[0].iter().enumerate() {
            match p {
                &Path::NS => { return (x, 0); },
                _ => { continue },
            }
        }
        panic!("could not find start point");
    }
    fn get (&self, pos: (usize, usize)) -> &Path {
        let (x, y) = pos;
        self.data.get(y)
            .and_then(|row| row.get(x))
            .unwrap_or_else(|| &Path::Empty)
    }
    fn intersection (&self, cursor: &mut Cursor) {
        let left = cursor.direction.left();
        let next_pos = cursor.next_pos(&left);
        let p = self.get(next_pos);

        match p {
            &Path::NS | &Path::EW => {
                cursor.turn(left);
            },
            _ => {
                let right = cursor.direction.right();
                cursor.turn(right);
            }
        };
        cursor.step();
    }
    fn traverse(&self) -> Cursor {
        let (x, y) = self.find_start_point();
        let mut cursor = Cursor::new(x, y);

        loop {
            let pos = cursor.pos();
            let p = self.get(pos);
            match p {
                &Path::Inter => {
                    self.intersection(&mut cursor);
                },
                &Path::Letter(ch) => {
                    cursor.visit(ch);
                    cursor.step();
                },
                &Path::NS | &Path::EW => {
                    cursor.step();
                },
                &Path::Empty => {
                    break;
                }
            }
        }

        cursor
    }
}
