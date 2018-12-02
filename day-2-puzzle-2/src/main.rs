use rayon::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;

fn are_close(a: &str, b: &str) -> bool {
    assert_eq!(a.len(), b.len());

    let num_common: usize = a
        .chars()
        .zip(b.chars())
        .map(|(a, b)| (a == b) as usize)
        .sum();

    num_common == a.len() - 1
}

fn common_characters(a: &str, b: &str) -> String {
    assert_eq!(a.len(), b.len());

    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

fn main() {
    let matches = clap::App::new("id-checksum")
        .arg(clap::Arg::with_name("input").required(false))
        .get_matches();

    let reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let indices = (0..lines.len())
        .into_par_iter()
        .flat_map(|i| ((i + 1)..lines.len()).into_par_iter().map(move |j| (i, j)))
        .find_any(|(i, j)| are_close(&lines[*i], &lines[*j]));

    match indices {
        Some((i, j)) => println!("{}", common_characters(&lines[i], &lines[j])),
        None => {
            println!("No close IDs!");
            std::process::exit(1);
        }
    }
}
