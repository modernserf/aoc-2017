extern crate regex;
use regex::Regex;
use std::ops::Add;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = include_str!("input.txt");
    let particles = parse(&contents);

    let closest_particle_index = find_closest_particle_index(&particles);
    println!("part 1: {}", closest_particle_index);

    let remaining_count = smash(&particles, 1000);
    println!("part 2: {}", remaining_count);
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
    fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    p: Point,
    v: Point,
    a: Point,
}

impl Particle {
    fn new(p: Point, v: Point, a: Point) -> Particle {
        Particle { p, v, a }
    }
    fn tick(&self) -> Particle {
        let next_v = self.v + self.a;
        let next_p = self.p + next_v;
        Particle {
            p: next_p,
            v: next_v,
            a: self.a,
        }
    }
    fn hash(&self) -> (i64, i64, i64) {
        (self.p.x, self.p.y, self.p.z)
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.p == other.p && self.v == other.v && self.a == other.a
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        if self.a.distance() != other.a.distance() {
            return Some(self.a.distance().cmp(&other.a.distance()));
        }
        if self.v.distance() != other.v.distance() {
            return Some(self.v.distance().cmp(&other.v.distance()));
        }
        if self.p.distance() != other.p.distance() {
            return Some(self.p.distance().cmp(&other.p.distance()))
        }
        return None
    }
}

fn parse (s: &str) -> Vec<Particle> {
    let re = Regex::new(
        r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$"
    ).unwrap();

    s.lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            re.captures_iter(line).map(|nums| {
                Particle::new(
                    Point::new(nums[1].parse::<i64>().unwrap(), nums[2].parse::<i64>().unwrap(), nums[3].parse::<i64>().unwrap()),
                    Point::new(nums[4].parse::<i64>().unwrap(), nums[5].parse::<i64>().unwrap(), nums[6].parse::<i64>().unwrap()),
                    Point::new(nums[7].parse::<i64>().unwrap(), nums[8].parse::<i64>().unwrap(), nums[9].parse::<i64>().unwrap()))
            })
            .nth(0).unwrap()
        })
        .collect::<Vec<Particle>>()
}


fn find_closest_particle_index(ps: &[Particle]) -> usize {
    ps.iter().enumerate().fold(0, |min_i, (i, particle)| {
        let min_particle = &ps[min_i];
        if particle < min_particle { i } else { min_i }
    })
}

fn smash (init_ps: &[Particle], times: usize) -> usize {
    let mut ps : Vec<Particle> = init_ps.to_vec();

    for _ in 0..times {
        let mut points = HashMap::new();
        let mut collisions = HashSet::new();
        for particle in ps {
            let key = particle.hash();
            if collisions.contains(&key) { continue; }
            if points.get(&key).is_some() {
                points.remove(&key);
                collisions.insert(key);
                continue;
            }
            points.insert(key, particle);
        }
        ps = points.values().map(|particle| particle.tick()).collect();
    }
    ps.len()
}
