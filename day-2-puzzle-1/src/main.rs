use rayon::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;

// Tuple of if the id contains two duplicate characters and if it contains three duplicate
// characters.
fn get_buckets(id: &str) -> (bool, bool) {
    assert!(id.is_ascii());

    let mut counts: [i8; 26] = [0; 26];
    let mut num_twos = 0;
    let mut num_threes = 0;

    for c in id.bytes() {
        assert!(c >= b'a' && c <= b'z');

        let n = (c - b'a') as usize;

        counts[n] += 1;

        assert!(counts[n] <= 3);

        if counts[n] == 3 {
            num_threes += 1;
            num_twos -= 1;
        } else if counts[n] == 2 {
            num_twos += 1;
        }
    }

    (num_twos > 0, num_threes > 0)
}

fn main() {
    let matches = clap::App::new("id-checksum")
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

    let sum: (i32, i32) = input_deck
        .as_str()
        .par_lines()
        .map(|line| {
            let buckets = get_buckets(&line);
            (buckets.0 as i32, buckets.1 as i32)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}", sum.0 * sum.1);
}
