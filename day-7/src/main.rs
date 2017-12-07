#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");

    let mut tree = NodeTree::new();

    for line in contents.lines() {
        if line.len() > 0 {
            let (name, weight, children) = parse_line(line);
            tree.insert(name, weight, children);
        }
    }

    let root = &tree.roots()[0];
    println!("part 1: {:?}", root);

    let w = tree.balanced_weight(&root);
    println!("part 2: {:?}", w);
}


#[derive(Debug)]
struct NodeTree {
    data: HashMap<String, (u32, Vec<String>)>,
}

impl NodeTree {
    fn new() -> NodeTree {
        NodeTree { data: HashMap::new() }
    }

    fn insert(&mut self, name: String, weight: u32, children: Vec<String>) {
        self.data.insert(name, (weight, children));
    }

    fn roots(&self) -> Vec<String> {
        let node_names : HashSet<&String> =
            self.data.keys().collect();
        let child_names : HashSet<&String> =
            self.data.values()
            .flat_map(|&(_, ref cs)| cs)
            .collect();

        let dif = node_names.difference(&child_names)
            .map(|k| k.to_lowercase())
            .collect::<Vec<String>>();
        dif
    }

    fn children_of(&self, id: &str) -> Option<Vec<String>> {
        self.data.get(id).map(|&(_, ref children)| children.to_vec())
    }

    fn weight_of(&self, id: &str) -> Option<u32> {
        self.data.get(id).map(|&(weight, _)| weight)
    }

    fn balanced_weight(&self, id: &str) -> Result<u32, u32> {
        let children = self.children_of(id).unwrap();
        let w = self.weight_of(id).unwrap();

        if children.len() == 0 { return Ok(w) }

        let mut child_weights = Vec::new();

        // propagate error
        for ch in children.iter() {
            let w_res = self.balanced_weight(ch);
            if let Ok(cw) = w_res  {
                child_weights.push(cw);
            } else {
                return w_res
            }
        }

        if let Some((wrong_index, offset)) = wrong_value(&child_weights) {
            let wrong_id = &children[wrong_index];
            let fixed_weight = self.weight_of(wrong_id).unwrap() - offset;
            Err(fixed_weight)
        } else {
            let sum : u32 = child_weights.iter().sum();
            Ok(w + sum)
        }
    }
}

fn wrong_value(xs: &[u32]) -> Option<(usize, u32)> {
    if xs.len() < 3 { return None; }

    let min = xs.iter().min().unwrap();
    let max = xs.iter().max().unwrap();

    let mut mins = Vec::new();
    let mut maxs = Vec::new();

    for (i, x) in xs.iter().enumerate() {
        if x == min { mins.push(i) }
        if x == max { maxs.push(i) }
    }

    if mins.len() == maxs.len() { return None; }
    if mins.len() > maxs.len() {
        Some((maxs[0], max - min))
    } else {
        Some((mins[0], min - max))
    }
}


fn parse_line(line: &str) -> (String, u32, Vec<String>) {
    if line.contains("->") {
        let split = line.split(" -> ").collect::<Vec<_>>();
        let head = split[0];
        let tail = split[1];
        let (name, weight) = parse_head(head);
        let children = parse_tail(tail);

        (name, weight, children)
    }  else {
        let (name, weight) = parse_head(line);
        (name, weight, vec![])
    }
}

fn parse_head(s: &str) -> (String, u32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+) \((\d+)\)").unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            let name = String::from(&cap[1]);
            let weight = cap[2].parse::<u32>().unwrap();
            (name, weight)
        })
        .nth(0).unwrap()
}

fn parse_tail(s: &str) -> Vec<String> {
    s.split(", ")
        .map(|w| { String::from(w) })
        .collect::<Vec<_>>()
}
