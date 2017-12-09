#[macro_use] extern crate nom;
use nom::{alpha, anychar};

fn main() {
    let line = include_str!("input.txt");
    // let line = "{<<<<>}";
    let root_node = parse_line(&line);
    let score = root_node.score();
    println!("part 1: {}", score);
    let garbage_size = root_node.garbage_size();
    println!("part 2: {}", garbage_size);
}

fn parse_line(line: &str) -> Node {
    let bytes: Vec<u8> = line.trim().bytes().collect();

    let (_, node) = group(&bytes).unwrap();
    node
}

#[derive(Debug)]
enum Node {
    Garbage(u32),
    Group(Vec<Node>),
}

impl Node {
    fn score_with_depth(&self, depth: u32) -> u32 {
        match self {
            &Node::Garbage(_) => 0,
            &Node::Group(ref nodes) => {
                let sum: u32 = nodes.iter().map(|node| {
                    node.score_with_depth(depth + 1)
                }).sum();
                depth + sum
            }
        }
    }
    fn score(&self) -> u32 {
        self.score_with_depth(1)
    }
    fn garbage_size(&self) -> u32 {
        match self {
            &Node::Garbage(size) => size,
            &Node::Group(ref nodes) => {
                nodes.iter().map(|node| { node.garbage_size() }).sum()
            }
        }
    }
}


named!(group<Node>,
    do_parse!(
        l:delimited!(
            op_grp,
            separated_list_complete!(
                comma,
                alt_complete!(group | garbage)),
            cl_grp) >>
        (Node::Group(l))));

named!(garbage<Node>,
    do_parse!(
        op_garb >>
        gs: many0!(garbage_char) >>
        cl_garb >>
        ({
            let sum = gs.iter().sum();
            Node::Garbage(sum)
        })));

named!(garbage_char<u32>,
    alt_complete!(
        do_parse!(bang >> anychar >> (0)) |
        do_parse!(alt!(op_garb | op_grp | cl_grp | comma | quot | qq) >> (1)) |
        letters));

named!(letters<u32>,
    do_parse!(a: alpha >> (a.len() as u32)));

named!(bang, tag!("!"));
named!(op_garb, tag!("<"));
named!(cl_garb, tag!(">"));
named!(op_grp, tag!("{"));
named!(cl_grp, tag!("}"));
named!(comma, tag!(","));
named!(quot, tag!("'"));
named!(qq, tag!("\""));
