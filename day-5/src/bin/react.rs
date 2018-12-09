use day_5::react_par;
use std::io::{prelude::*, BufReader};

fn main() {
    let matches = clap::App::new("react")
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

    let mut polymer = Vec::from(polymer.as_bytes());
    assert!(polymer
        .iter()
        .cloned()
        .all(|c| (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')));

    println!("{}", react_par(&mut polymer[..]));
}
