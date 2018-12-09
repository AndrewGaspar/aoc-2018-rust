use std::io::{prelude::*, BufReader};

use day_5::react_par;
use rayon::prelude::*;

fn main() {
    let matches = clap::App::new("shortest-polymer")
        .arg(clap::Arg::with_name("input").required(false))
        .get_matches();

    let mut reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    let mut polymer = Default::default();
    reader.read_to_string(&mut polymer).unwrap();
    let polymer = polymer.trim();

    let polymer = Vec::from(polymer.as_bytes());
    assert!(polymer
        .iter()
        .cloned()
        .all(|c| (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')));

    let shortest = (0u8..26)
        .into_par_iter()
        .map(|c1| {
            let mut polymer: Vec<_> = polymer
                .iter()
                .cloned()
                .filter(|&c2| {
                    if c2 >= b'a' && c2 <= b'z' {
                        c2 != (b'a' + c1)
                    } else {
                        c2 != (b'A' + c1)
                    }
                })
                .collect();

            react_par(&mut polymer[..])
        })
        .min()
        .unwrap();

    println!("{}", shortest);
}
