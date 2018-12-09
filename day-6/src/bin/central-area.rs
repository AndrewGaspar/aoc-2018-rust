use std::cmp::{max, min};
use std::io::{prelude::*, BufReader};

use day_6::Point;
use rayon::prelude::*;

use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

fn main() {
    let matches = clap::App::new("max-finite-area")
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

    let mut points = input_deck
        .as_str()
        .par_lines()
        .map(Point::from_line)
        .collect::<Vec<_>>();

    let (lower_bounds, upper_bounds) = points.par_iter().map(|p| (*p, *p)).reduce(
        || {
            (
                Point {
                    x: i32::max_value(),
                    y: i32::max_value(),
                },
                Point {
                    x: i32::min_value(),
                    y: i32::min_value(),
                },
            )
        },
        |a, b| {
            (
                Point {
                    x: min(a.0.x, b.0.x),
                    y: min(a.0.y, b.0.y),
                },
                Point {
                    x: max(a.1.x, b.1.x),
                    y: max(a.1.y, b.1.y),
                },
            )
        },
    );

    let x_dims = upper_bounds.x - lower_bounds.x + 1;
    let y_dims = upper_bounds.y - lower_bounds.y + 1;

    points.par_iter_mut().for_each(|p| {
        // Shift the points to make this easier
        p.x -= lower_bounds.x;
        p.y -= lower_bounds.y;
    });

    let count: usize = (0..x_dims)
        .into_par_iter()
        .flat_map(|i| (0..y_dims).into_par_iter().map(move |j| (i, j)))
        .map(|(x, y)| points.iter().map(|p| p.dist_from(&Point { x, y })).sum())
        .filter(|dist: &i32| *dist < 10000)
        .count();

    println!("{}", count);
}
