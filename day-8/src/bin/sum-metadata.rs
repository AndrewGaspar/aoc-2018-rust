use std::io::{prelude::*, BufReader};

use day_8::NodeTree;

fn main() {
    let matches = clap::App::new("sum-metadata")
        .arg(clap::Arg::with_name("input").required(false))
        .get_matches();

    let reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    let tree = NodeTree::from_reader(reader);
    let sum: u32 = tree
        .nodes
        .iter()
        .map(|n| n.metadata.iter().cloned().map(|i| i as u32).sum::<u32>())
        .sum();

    println!("{}", sum);
}
