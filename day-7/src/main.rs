#[macro_use] extern crate nom;
use nom::{alpha, digit, space};

use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");

    let tree = contents.lines()
        .fold(NodeTree::new(), |mut t, line| {
            if line.len() > 0 {
                let (name, weight, children) = parse_line(line);
                t.insert(name, weight, children);
            }
            t
        });

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
    let bytes: Vec<u8> = line.bytes().collect();

    let (_, (name, weight, children)) = expr(&bytes).unwrap();
    (name, weight, children)
}

named!(expr<(String, u32, Vec<String>)>,
    alt_complete!(
        do_parse!(
            h:head >> space >> tag!("->") >> space >> t:tail >>
            (h.0, h.1, t)) |
        do_parse!(
            h:head >>
            (h.0, h.1, { Vec::new() })
        )));


named!(head<(String, u32)>,
    do_parse!(
        l: label >> space >> w: weight >>
        (l, w)
    ));

named!(tail<Vec<String>>,
    separated_nonempty_list_complete!(tag!(", "), label));

named!(label<String>,
    flat_map!(call!(alpha), parse_to!(String)));

named!(weight<u32>,
    delimited!(tag!("("), num, tag!(")")));

named!(num<u32>,
    flat_map!(call!(digit), parse_to!(u32)));
