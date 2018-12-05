use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

use rayon::prelude::*;
use rayon::slice::ParallelSliceMut;
use regex::Regex;

#[derive(Debug)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
enum EventType {
    BeginShift { guard_id: u32 },
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Event {
    date: Date,
    event_type: EventType,
}

struct EventParser {
    event_regex: Regex,
    message_regex: Regex,
}

impl EventParser {
    fn new() -> Self {
        Self {
            event_regex: Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap(),
            message_regex: Regex::new(r"^Guard #(\d+) begins shift$").unwrap(),
        }
    }

    fn parse_event(&self, line: &str) -> Event {
        let captures = self.event_regex.captures(&line).expect("Event was invalid");

        let year = captures[1].parse().unwrap();
        let month = captures[2].parse().unwrap();
        let day = captures[3].parse().unwrap();
        let hour = captures[4].parse().unwrap();
        let minute = captures[5].parse().unwrap();

        let message = &captures[6];
        let event_type = match message {
            "falls asleep" => EventType::FallsAsleep,
            "wakes up" => EventType::WakesUp,
            _ => {
                let captures = self
                    .message_regex
                    .captures(&message)
                    .expect(&format!("Guard message was invalid: {}", message));
                EventType::BeginShift {
                    guard_id: captures[1].parse().unwrap(),
                }
            }
        };

        Event {
            date: Date {
                year,
                month,
                day,
                hour,
                minute,
            },
            event_type,
        }
    }
}

fn main() {
    let matches = clap::App::new("sleeping-guards")
        .arg(clap::Arg::with_name("input").required(false))
        .get_matches();

    let mut reader: BufReader<Box<Read>> = match matches.value_of("input") {
        Some(input_file) => BufReader::new(Box::new(
            std::fs::File::open(input_file).expect("Could not open input file"),
        )),
        None => BufReader::new(Box::new(std::io::stdin())),
    };

    let parser = EventParser::new();

    let mut input_deck = String::new();
    reader.read_to_string(&mut input_deck).unwrap();

    let mut events: Vec<_> = input_deck
        .as_str()
        .par_lines()
        .map(|line| parser.parse_event(&line))
        .collect();

    events.as_mut_slice().par_sort_by_key(|a| {
        fn date_time_key(date: &Date) -> usize {
            let mut key: usize = 0;
            key += date.year as usize;
            key *= 100;
            key += date.month as usize;
            key *= 100;
            key += date.day as usize;
            key *= 100;
            key += date.hour as usize;
            key *= 100;
            key += date.minute as usize;
            key
        }

        date_time_key(&a.date)
    });

    let mut guard_events = HashMap::new();

    let mut current_guard: Option<u32> = None;
    let mut sleep_start = None;
    let mut sleep_ranges = Some(Vec::new());
    for event in &events {
        match event.event_type {
            EventType::BeginShift { guard_id } => {
                if let Some(last_guard_id) = current_guard.take() {
                    let existing_event: Option<&mut Vec<_>> = guard_events.get_mut(&last_guard_id);

                    if let Some(ranges) = existing_event {
                        ranges.append(&mut sleep_ranges.take().unwrap());
                    } else {
                        guard_events.insert(last_guard_id, sleep_ranges.take().unwrap());
                    }
                }

                assert!(sleep_start.is_none());
                current_guard = Some(guard_id);
                sleep_ranges = Some(Vec::new());
            }
            EventType::FallsAsleep => {
                assert!(sleep_start.is_none());
                assert_eq!(0, event.date.hour);
                sleep_start = Some(event.date.minute);
            }
            EventType::WakesUp => {
                assert_eq!(0, event.date.hour);
                sleep_ranges
                    .as_mut()
                    .unwrap()
                    .push(sleep_start.take().unwrap()..event.date.minute);
            }
        }
    }

    if let Some(last_guard_id) = current_guard.take() {
        let existing_event: Option<&mut Vec<_>> = guard_events.get_mut(&last_guard_id);

        if let Some(ranges) = existing_event {
            ranges.append(&mut sleep_ranges.take().unwrap());
        } else {
            guard_events.insert(last_guard_id, sleep_ranges.take().unwrap());
        }
    }

    let sleepiest = guard_events
        .iter()
        .max_by_key(|(_, sleep_times)| {
            sleep_times
                .iter()
                .map(|range| (range.end - range.start) as u32)
                .sum::<u32>()
        })
        .expect("Zero guards...");

    let mut sleep_counts: [u32; 60] = [0; 60];
    for range in sleepiest.1.iter() {
        for x in range.clone() {
            sleep_counts[x as usize] += 1;
        }
    }

    let (sleepiest_minute, _) = sleep_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, counts)| *counts)
        .unwrap();

    println!("{}", *sleepiest.0 * sleepiest_minute as u32);
}
