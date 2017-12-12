use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let content = include_str!("input.txt");
    let network = Network::from_string(content);
    println!("part 1: {}", network.count_linked(0));
    println!("part 2: {}", network.count_groups());
}


struct Network {
    data: HashMap<u32, Vec<u32>>,
}

impl Network {
    fn from_string(s: &str) -> Network {
        let data: HashMap<u32, Vec<u32>> = s.lines()
            .filter_map(|line| {
                if line.len() > 0 { Some(parse_line(line)) } else { None }
            })
            .collect();

        Network { data }
    }
    fn find_group_inner(&self, id: &u32, mut visited: &mut HashSet<u32>) {
        visited.insert(*id);
        if let Some(children) = self.data.get(id) {
            for child in children {
                if !visited.contains(child) {
                    self.find_group_inner(child, &mut visited);
                }
            }
        }
    }
    fn find_group(&self, id: u32) -> HashSet<u32> {
        let mut visited = HashSet::new();
        self.find_group_inner(&id, &mut visited);
        visited
    }
    fn count_linked(&self, id: u32) -> u32 {
        self.find_group(id).len() as u32
    }
    fn count_groups(&self) -> u32 {
        let mut visited = HashSet::new();
        self.data.keys().fold(0, |count, id| {
            if visited.contains(id) {
                count
            } else {
                self.find_group_inner(id, &mut visited);
                count + 1
            }
        })
    }
}


fn parse_line(line: &str) -> (u32, Vec<u32>) {
    let parts: Vec<String> = line.split("<->").map(|s| String::from(s)).collect();
    let root = parts[0].trim().parse::<u32>().expect("root");
    let links = parts[1].trim().split(", ")
        .map(|p| p.parse::<u32>().expect("item"))
        .collect::<Vec<u32>>();
    (root, links)
}
