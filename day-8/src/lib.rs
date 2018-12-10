use std::io::{prelude::*, BufReader};

use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub children: Vec<u32>,
    pub metadata: Vec<u8>,
}

struct NodeHeader {
    num_children: u8,
    num_metadata: u8,
    id: Option<u32>,
}

pub struct NodeTree {
    pub nodes: Vec<Node>,
}

impl NodeTree {
    pub fn from_reader(reader: BufReader<Box<Read>>) -> Self {
        let mut reader = NodeTreeReader::new(reader);

        let mut node_queue = vec![reader.next_header()];

        let mut nodes = vec![];

        while !node_queue.is_empty() {
            let current = node_queue.last_mut().unwrap();
            if current.id.is_none() {
                current.id = Some(nodes.len() as u32);
                nodes.push(Node {
                    children: Vec::with_capacity(current.num_children as usize),
                    metadata: Vec::with_capacity(current.num_metadata as usize),
                });
            }

            if current.num_children > 0 {
                current.num_children -= 1;

                let next_id = nodes.len() as u32;
                nodes[current.id.unwrap() as usize].children.push(next_id);

                node_queue.push(reader.next_header());
            } else {
                for _ in 0..current.num_metadata {
                    nodes[current.id.unwrap() as usize]
                        .metadata
                        .push(reader.next_int());
                }

                node_queue.pop();
            }
        }

        NodeTree { nodes }
    }

    pub fn node_value(&self, i: u32) -> u32 {
        if self.nodes[i as usize].children.is_empty() {
            self.nodes[i as usize]
                .metadata
                .iter()
                .cloned()
                .map(|i| i as u32)
                .sum()
        } else {
            self.nodes[i as usize]
                .metadata
                .par_iter()
                .map(|m| {
                    let m = m - 1;
                    let node = &self.nodes[i as usize];
                    if (m as usize) < node.children.len() {
                        self.node_value(node.children[m as usize] as u32)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

struct NodeTreeReader {
    reader: std::io::Split<BufReader<Box<Read>>>,
}

impl NodeTreeReader {
    pub fn new(reader: BufReader<Box<Read>>) -> Self {
        Self {
            reader: reader.split(b' '),
        }
    }

    fn next_header(&mut self) -> NodeHeader {
        let num_children = self.next_int();
        let num_metadata = self.next_int();

        NodeHeader {
            num_children,
            num_metadata,
            id: None,
        }
    }

    fn next_int<F: std::str::FromStr>(&mut self) -> F
    where
        <F as std::str::FromStr>::Err: std::fmt::Debug,
    {
        String::from_utf8(self.reader.next().unwrap().unwrap())
            .unwrap()
            .parse()
            .unwrap()
    }
}
