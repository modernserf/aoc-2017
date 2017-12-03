use std::collections::HashMap;

fn main() {
    let input = 325489;
    let addr = SpiralAddress::at_index(input);
    println!("part 1: {}", addr.taxi_distance());

    let larger_value = find_larger_value(input);
    println!("part 2: {}", larger_value);
}

struct SpiralAddress {
    offset: i32,
    ring: i32,
}

impl SpiralAddress {
    fn at_index (index: i32) -> SpiralAddress {
        // width: 1 3 5 7 9 ...
        // ring size: 1 8 16 24 ...
        let mut offset = index - 1;
        let mut ring = 0;

        while offset >= SpiralAddress::perimeter_at_ring(ring) {
            offset -= SpiralAddress::perimeter_at_ring(ring);
            ring += 1;
        }

        SpiralAddress { offset, ring }
    }
    fn perimeter_at_ring (ring: i32) -> i32 {
        if ring == 0 {
            1
        } else {
            ring * 8
        }
    }
    // this is *essentially* a polar -> cartesian projection, right?
    // feel like there should be a direct algorithmic transformation
    fn to_point (self) -> (i32, i32) {
        if self.ring == 0 { return (0, 0) }

        // zero offset
        let f = self.ring - 1;

        let o = self.offset;
        // from zero offset to top right corner (width - 1)
        let w = self.ring * 2;

        // right
        if o < w { (self.ring, o - f) }
        // top
        else if o < w * 2 { (w - f - 2 - (o - w), self.ring) }
        // left
        else if o < w * 3 { (-self.ring, w - f - 2 - (o - w * 2)) }
        // bottom
        else { (o - f - (3 * w), -self.ring) }
    }
    fn taxi_distance (self) -> i32 {
        let (x, y) = self.to_point();
        x.abs() + y.abs()
    }
}


fn find_larger_value (val: i32) -> i32 {
    let mut mem = HashMap::new();
    mem.insert((0, 0), 1);

    for i in 2..100 {
        let addr = SpiralAddress::at_index(i);
        let point = addr.to_point();
        let sum = find_neighbors_sum(point, &mem);
        if sum > val { return sum }

        mem.insert(point, sum);
    }

    panic!("didn't find larger address");
}


fn find_neighbors_sum (point: (i32, i32), m: &HashMap<(i32, i32), i32>) -> i32 {
    let (x, y) = point;

    [
        (-1,1),  (0,1),  (1,1),
        (-1,0),          (1,0),
        (-1,-1), (0,-1), (1,-1),
    ].iter()
    .map(|&(dx, dy)| (x + dx, y + dy))
    .filter_map(|p| m.get(&p))
    .sum()
}
