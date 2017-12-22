use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let mut plane = Plane::from_string(contents);
    for _ in 0..10_000 {
        plane.step();
    }
    println!("part 1: {}", plane.infected_count);

    let mut plane = Plane::from_string(contents);
    for _ in 0..10_000_000 {
        plane.step_evolved();
    }
    println!("part 2: {}", plane.infected_count);
}

#[derive(Debug, Copy, Clone)]
enum NodeStatus { Clean, Weakened, Infected, Flagged }

#[derive(Debug)]
enum Direction { N, S, E, W }

impl Direction {
    fn right(&self) -> Direction {
        match self {
            &Direction::N => Direction::E,
            &Direction::E => Direction::S,
            &Direction::S => Direction::W,
            &Direction::W=> Direction::N,
        }
    }
    fn left(&self) -> Direction {
        // #[derive(Zoolander)]
        self.right().right().right()
    }
    fn reverse(&self) -> Direction {
        self.right().right()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new (x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn offset (&self, dx: i32, dy: i32) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }
    fn forward(&self, dir: &Direction) -> Point {
        match dir {
            &Direction::N => self.offset(0, -1),
            &Direction::E => self.offset(1, 0),
            &Direction::S => self.offset(0, 1),
            &Direction::W => self.offset(-1, 0),
        }
    }
}


struct Plane {
    data: HashMap<Point, NodeStatus>,
    infected_count: u32,
    position: Point,
    direction: Direction,
}

impl Plane {
    fn from_string (s: &str) -> Plane {
        let mut height = 0;
        let mut width = 0;
        let mut data = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if height == 0 { width += 1; }
                if ch == '#' {
                    let point = Point::new(x as i32, y as i32);
                    data.insert(point, NodeStatus::Infected);
                }
            }
            height += 1;
        }

        let cx = width / 2;
        let cy = height / 2;

        Plane {
            data,
            infected_count: 0,
            position: Point::new(cx as i32, cy as i32),
            direction: Direction::N,
        }
    }
    fn get(&self, point: &Point) -> NodeStatus {
        *self.data.get(point).unwrap_or_else(|| &NodeStatus::Clean)
    }
    fn step(&mut self) {
        match self.get(&self.position) {
            NodeStatus::Clean => {
                self.direction = self.direction.left();
                self.data.insert(self.position, NodeStatus::Infected);
                self.infected_count += 1;
            },
            _ => {
                self.direction = self.direction.right();
                self.data.insert(self.position, NodeStatus::Clean);
            }
        }
        self.position = self.position.forward(&self.direction);
    }
    fn step_evolved(&mut self) {
        match self.get(&self.position) {
            NodeStatus::Clean => {
                self.direction = self.direction.left();
                self.data.insert(self.position, NodeStatus::Weakened);
            },
            NodeStatus::Weakened => {
                self.data.insert(self.position, NodeStatus::Infected);
                self.infected_count += 1;
            },
            NodeStatus::Infected => {
                self.direction = self.direction.right();
                self.data.insert(self.position, NodeStatus::Flagged);
            },
            NodeStatus::Flagged => {
                self.direction = self.direction.reverse();
                self.data.insert(self.position, NodeStatus::Clean);
            },
        }

        self.position = self.position.forward(&self.direction);
    }
}
