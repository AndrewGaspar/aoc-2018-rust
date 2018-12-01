use rayon::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let matches = clap::App::new("frequency-calculator")
        .arg(clap::Arg::with_name("input").required(false))
        .get_matches();

    let mut reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    let mut input_deck = String::new();
    reader.read_to_string(&mut input_deck).unwrap();

    let sum: i32 = input_deck
        .as_str()
        .par_lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum();

    println!("{}", sum);
}
