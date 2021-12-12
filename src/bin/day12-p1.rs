use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use aoc_2021::load_data_file;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
enum Node {
    Big(String),
    Small(String),
    Start,
    End,
}

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines().map(|l| l.split('-').collect_vec()).map(|v| {
        let parse = |s| -> Node {
            match s {
                "start" => Node::Start,
                "end" => Node::End,
                s if s.chars().all(char::is_uppercase) => Node::Big(s.to_string()),
                s if s.chars().all(char::is_lowercase) => Node::Small(s.to_string()),
                _ => panic!(),
            }
        };

        (parse(v[0]), parse(v[1]))
    });

    let mut nodes = HashSet::new();
    let mut rverts = Vec::new();
    for (l, r) in lines {
        nodes.insert(l.clone());
        nodes.insert(r.clone());

        rverts.push((l.clone(), r.clone()));
        rverts.push((r, l));
    }

    let grouped_verts = rverts.iter().sorted().group_by(|(l, _)| l);

    let mut verts: HashMap<Node, Vec<Node>> = HashMap::new();
    for (group, val) in grouped_verts.into_iter() {
        for node in val {
            if let Some(v) = verts.get(group) {
                let mut v2 = v.clone();
                v2.push(node.1.clone());
                verts.insert(group.clone(), v2);
            } else {
                verts.insert(group.clone(), vec![node.1.clone()]);
            }
        }
    }
    println!("{:?}", verts);

    let paths = explore(Node::Start, vec![], HashSet::new(), &verts);

    for path in paths.iter() {
        println!("{:?}", &path)
    }
    println!("{}", paths.len());

    return Ok(());
}

type Path = Vec<Node>;
fn explore(
    start: Node,
    mut path: Path,
    mut little_visited: HashSet<Node>,
    verts: &HashMap<Node, Vec<Node>>,
) -> Vec<Path> {
    path = [path, vec![start.clone()]].concat();
    if Node::End == start {
        return vec![path];
    }

    if let Node::Small(_) = start {
        little_visited.insert(start.clone());
    }

    let mut paths = vec![];
    for next in verts.get(&start.clone()).unwrap() {
        match next {
            Node::Big(_) | Node::End => {
                paths.append(&mut explore(
                    next.clone(),
                    path.clone(),
                    little_visited.clone(),
                    verts,
                ));
            }
            Node::Small(_) => {
                if !little_visited.contains(next) {
                    paths.append(&mut explore(
                        next.clone(),
                        path.clone(),
                        little_visited.clone(),
                        verts,
                    ));
                }
            }
            Node::Start => {
                continue;
            }
        }
    }

    return paths;
}
