use std::collections::HashMap;

fn main() {
    let content = include_str!("input.txt");
    // let content = "0: 3\n1: 2\n4: 4\n6: 4\n";

    let firewall = FireWall::from_string(content);
    println!("part 1: {}", firewall.score_direct_path());

    println!("part 2: {}", firewall.min_delay_not_caught());
}

struct FireWall {
    data: HashMap<u32, u32>,
    max_id: u32,
}

impl FireWall {
    fn from_string(s: &str) -> FireWall {
        let data: HashMap<u32, u32> = s.lines()
            .filter_map(|l| {
                if l.len() == 0 { return None }
                let parts: Vec<u32> = l.split(": ")
                    .filter_map(|s| String::from(s).parse::<u32>().ok())
                    .collect();
                Some((parts[0], parts[1]))
            })
            .collect();

        let max_id = *data.keys().max().unwrap();

        FireWall { data, max_id }
    }
    fn score_severity(&self, layer: u32, tick: u32) -> Option<u32> {
        self.data.get(&layer)
            .and_then(|range| {
                if back_and_forth(tick, *range) == 0 {
                    Some(layer * range)
                } else {
                    None
                }
            })
    }
    fn score_direct_path(&self) -> u32 {
        let min = 0;
        let max = self.max_id + 1;

        (min..max).fold(0, |score, layer| {
            self.score_severity(layer, layer)
                .map(|s| score + s)
                .unwrap_or_else(|| score)
        })
    }
    fn cleared_layers(&self, delay: u32) -> bool {
        let min = 0;
        let max = self.max_id + 1;
        for i in min..max {
            if self.score_severity(i, i + delay).is_some() {
                return false;
            }
        }
        true
    }
    fn min_delay_not_caught(&self) -> u32 {
        for delay in 0..10_000_000 {
            if self.cleared_layers(delay) {
                return i;
            }
        }
        panic!("could not avoid capture in 10m picoseconds")
    }
}

fn back_and_forth(tick: u32, range: u32) -> u32 {
    let period = range * 2 - 2;
    let mod_tick = tick % period;
    if mod_tick < range {
        mod_tick
    } else {
        2 * range - mod_tick
    }
}
