use std::io::prelude::*;
use std::io::BufReader;
use std::str;

use ndarray::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

fn read_number(line: &[u8]) -> (usize, u32) {
    let end_index = (0..line.len())
        .find(|x| line[*x] < b'0' || line[*x] > b'9')
        .unwrap_or(line.len());

    (
        end_index,
        unsafe { str::from_utf8(&line[0..end_index]) }
            .unwrap()
            .parse()
            .unwrap(),
    )
}

fn parse_claim(line: &str) -> Claim {
    assert!(line.is_ascii());

    let line = line.as_bytes();

    assert_eq!(b'#', line[0]);

    let mut next_index = 1;

    let (end, id) = read_number(&line[next_index..]);
    next_index += end;

    assert_eq!(b" @ "[..], line[next_index..next_index + 3]);
    next_index += 3;

    let (end, x) = read_number(&line[next_index..]);
    next_index += end;

    assert_eq!(b',', line[next_index]);
    next_index += 1;

    let (end, y) = read_number(&line[next_index..]);
    next_index += end;

    assert_eq!(b": "[..], line[next_index..next_index + 2]);
    next_index += 2;

    let (end, width) = read_number(&line[next_index..]);
    next_index += end;

    assert_eq!(b'x', line[next_index]);
    next_index += 1;

    let (end, height) = read_number(&line[next_index..]);
    next_index += end;

    assert_eq!(line.len(), next_index);

    Claim {
        id,
        x,
        y,
        width,
        height,
    }
}

fn overlapped_range(a: std::ops::Range<u32>, b: std::ops::Range<u32>) -> bool {
    a.start < b.end && a.end > b.start
}

fn overlapped(a: &Claim, b: &Claim) -> bool {
    overlapped_range(a.x..a.x + a.width, b.x..b.x + b.width)
        && overlapped_range(a.y..a.y + a.height, b.y..b.y + b.height)
}

fn main() {
    let matches = clap::App::new("intersections")
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

    let claims: Vec<_> = input_deck
        .as_str()
        .par_lines()
        .map(|line| parse_claim(line))
        .collect();

    let claim_idx = (0..claims.len())
        .into_par_iter()
        .find_any(|i| {
            !(0..claims.len())
                .into_par_iter()
                .filter(|j| j != i)
                .any(|j| overlapped(&claims[*i], &claims[j]))
        })
        .expect("Could not find a claim that doesn't overlap with any other claim");

    println!("{}", claims[claim_idx].id);
}
