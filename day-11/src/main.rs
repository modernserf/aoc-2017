fn main() {
    let contents = include_str!("input.txt");
    // let contents = "se,sw,se,sw,sw";

    let init_state = (HexCoord::new(0, 0), 0);

    let (pos, max_distance) = contents.trim().split(",")
        .fold(init_state, |(coord, distance), s|  {
            let next_coord = coord.inc(s);
            let max_distance = distance.max(next_coord.distance());
            (next_coord, max_distance)
        });
    println!("part 1: {}", pos.distance());
    println!("part 2: {}", max_distance);
}

struct HexCoord {
    x: i32,
    y: i32,
}

//      0, -1
// -1, 0    1, -1
//      0, 0
// -1, 1    1, 0
//      0, 1

impl HexCoord {
    fn new (x: i32, y: i32) -> HexCoord {
        HexCoord { x, y }
    }
    fn inc (&self, s: &str) -> HexCoord {
        match s {
            "nw" => HexCoord { x: self.x - 1,   y: self.y },
            "n"  => HexCoord { x: self.x,       y: self.y - 1 },
            "ne" => HexCoord { x: self.x + 1,   y: self.y - 1 },
            "sw" => HexCoord { x: self.x - 1,   y: self.y + 1 },
            "s"  => HexCoord { x: self.x,       y: self.y + 1 },
            "se" => HexCoord { x: self.x + 1,   y: self.y },
            _    => HexCoord { x: self.x,       y: self.y },
        }
    }
    fn distance(&self) -> i32 {
        let cube_z = 0 -self.x - self.y;
        (self.x.abs() + self.y.abs() + cube_z.abs()) / 2
    }
}
