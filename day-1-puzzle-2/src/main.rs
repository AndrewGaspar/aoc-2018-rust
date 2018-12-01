use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Creates a CLI app
    let matches = clap::App::new("frequency-repetition-finder")
        // Input file (optional - uses stdin if unspecified)
        .arg(clap::Arg::with_name("input").required(false))
        // max-repetitions controls the number of times the frequency list is cycled to find
        // duplicates.
        .arg(
            clap::Arg::with_name("max-repetitions")
                .takes_value(true)
                .short("n")
                .long("max_repetitions")
                .default_value("1000")
                .validator(|reps| {
                    if let Ok(x) = reps.parse::<usize>() {
                        if x > 0 {
                            return Ok(());
                        }
                    }

                    Err("max-repetitions must be a positive integer".to_string())
                }),
        )
        .get_matches();

    let max_repetitions = matches
        .value_of("max-repetitions")
        .map(|val| val.parse::<usize>().unwrap())
        .unwrap();

    // Opens either the input file or stdin to receive the list of frequency deltas
    let reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    // Collect the list of frequencies so we can endlessly cycle the iterator. `Lines` is consumed
    // when the iterator completes.
    let frequency_deltas: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    // A list of all observed frequencies.
    let mut observed_frequencies = vec![0];

    // Cycles over the list of frequency_deltas up to max_repetitions iterations of the list,
    // producing the current sum of the frequency deltas at each iteration.
    let frequencies = frequency_deltas
        .iter()
        .cycle()
        .take(frequency_deltas.len() * max_repetitions)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        });

    // Loops through the intermediate frequencies, storing the observed frequencies and exiting
    // once a repeated frequency is observed.
    for frequency in frequencies {
        if observed_frequencies.contains(&frequency) {
            println!("{}", frequency);
            std::process::exit(0);
        }

        observed_frequencies.push(frequency);
    }

    // Did not find a duplicate frequency. Prints out a suggestion to increase the number of
    // iterations through the frequency list.
    eprintln!(
        "No duplicate frequency was found in {} cycles of the frequencies list. Consider tuning \
         the -n option.",
        max_repetitions
    );
    std::process::exit(1);
}
