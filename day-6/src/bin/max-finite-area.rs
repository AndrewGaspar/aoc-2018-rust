use std::cmp::{max, min};
use std::io::{prelude::*, BufReader};

use day_6::Point;
use rayon::prelude::*;

use ndarray::prelude::*;

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

    let points = input_deck
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

    let box_dims = [
        (upper_bounds.x - lower_bounds.x + 1) as usize,
        (upper_bounds.y - lower_bounds.y + 1) as usize,
    ];

    #[derive(Copy, Clone, Debug)]
    enum Dist {
        Closest { point: usize, distance: i32 },
        Many { distance: i32 },
    }

    let closests = points
        .par_iter()
        .cloned()
        .map(|mut p| {
            // Shift the points to make this easier
            p.x -= lower_bounds.x;
            p.y -= lower_bounds.y;
            p
        })
        .enumerate()
        .fold(
            || Array2::default(box_dims),
            |mut closests: Array2<Option<Dist>>, point: (usize, Point)| {
                for ((x, y), marker) in closests.indexed_iter_mut() {
                    let distance = point.1.dist_from(&Point {
                        x: x as i32,
                        y: y as i32,
                    });

                    match marker {
                        Some(Dist::Many {
                            distance: other_distance,
                        }) => {
                            if distance < *other_distance {
                                *marker = Some(Dist::Closest {
                                    point: point.0,
                                    distance,
                                })
                            }
                        }
                        Some(Dist::Closest {
                            distance: other_distance,
                            ..
                        }) => {
                            if distance == *other_distance {
                                *marker = Some(Dist::Many { distance });
                            } else if distance < *other_distance {
                                *marker = Some(Dist::Closest {
                                    point: point.0,
                                    distance,
                                })
                            }
                        }
                        None => {
                            *marker = Some(Dist::Closest {
                                point: point.0,
                                distance,
                            });
                        }
                    }
                }

                closests
            },
        )
        .reduce(
            || Array2::default(box_dims),
            |mut closests_a, closests_b| {
                ndarray::Zip::from(&mut closests_a)
                    .and(&closests_b)
                    .apply(|a, &b| match a {
                        Some(Dist::Many {
                            distance: distance_a,
                        }) => {
                            if let Some(Dist::Closest {
                                distance: distance_b,
                                ..
                            }) = b
                            {
                                if distance_b < *distance_a {
                                    *a = b;
                                }
                            }
                        }
                        Some(Dist::Closest {
                            distance: distance_a,
                            ..
                        }) => match b {
                            Some(Dist::Many {
                                distance: distance_b,
                            }) => {
                                if distance_b < *distance_a {
                                    *a = b;
                                }
                            }
                            Some(Dist::Closest {
                                distance: distance_b,
                                ..
                            }) => {
                                if distance_b == *distance_a {
                                    *a = Some(Dist::Many {
                                        distance: distance_b,
                                    });
                                } else if distance_b < *distance_a {
                                    *a = b;
                                }
                            }
                            None => {}
                        },
                        None => *a = b,
                    });

                closests_a
            },
        );

    let mut is_disqualified = vec![false; points.len()];

    // Once an area reaches the "edge" of the bounding box, it's home free: the closest point going
    // outward will always be that area. So let
    for disqualified in closests
        .row(0)
        .iter()
        .chain(closests.row(closests.rows() - 1).iter())
        .chain(closests.column(0).iter())
        .chain(closests.column(closests.cols() - 1).iter())
        .filter_map(|closest| match closest {
            Some(Dist::Many { .. }) => None,
            Some(Dist::Closest { point, .. }) => Some(*point),
            None => panic!("Not all grid points were filled!"),
        })
    {
        is_disqualified[disqualified] = true;
    }

    let mut area_counts = vec![0; points.len()];
    for closest in &closests {
        match closest {
            Some(Dist::Closest { point, .. }) => {
                area_counts[*point] += 1;
            }
            _ => {}
        }
    }

    let (point, area) = area_counts
        .par_iter()
        .enumerate()
        .zip(is_disqualified.par_iter())
        .filter_map(|((point, &count), &disqualified)| {
            if disqualified {
                None
            } else {
                Some((point, count))
            }
        })
        .max_by_key(|p| p.1)
        .unwrap();

    println!("{}: {}", point, area);
}
