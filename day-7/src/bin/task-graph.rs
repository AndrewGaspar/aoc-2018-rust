use std::io::{prelude::*, BufReader};

use day_7::{Task, TaskDependency};
use rayon::prelude::*;

fn main() {
    let matches = clap::App::new("task-graph")
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

    let dependencies = input_deck
        .as_str()
        .par_lines()
        .map(TaskDependency::from_line)
        .collect::<Vec<_>>();

    let mut tasks = vec![Task::default(); 26];
    for dep in &dependencies {
        tasks[dep.target as usize].dependencies.push(dep.dependency);
        tasks[dep.dependency as usize].dependents.push(dep.target);

        tasks[dep.target as usize].incomplete = true;
        tasks[dep.dependency as usize].incomplete = true;
    }

    // sort and dedup dependencies and dependents in task list
    for task in &mut tasks {
        task.dependencies.sort();
        task.dependencies.dedup();

        task.dependents.sort();
        task.dependents.dedup();
    }

    let mut num_completed = tasks.iter().filter(|t| !t.incomplete).count();
    let mut completion_order = String::new();

    let mut removals = vec![];
    while num_completed != 26 {
        let task_idx = tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.incomplete)
            .find(|(_, t)| t.dependencies.is_empty())
            .map(|(i, _)| i)
            .expect("Exhausted task list too quickly") as u8;

        removals.clear();
        for dependent in &tasks[task_idx as usize].dependents {
            let remove_at = tasks[*dependent as usize]
                .dependencies
                .binary_search(&task_idx)
                .expect("Task dependency wasn't recorded");
            removals.push((*dependent, remove_at));
        }

        for removal in &removals {
            tasks[removal.0 as usize].dependencies.remove(removal.1);
        }

        tasks[task_idx as usize].incomplete = false;

        completion_order += unsafe { String::from_utf8_unchecked(vec![b'A' + task_idx]) }.as_str();

        num_completed += 1;
    }

    println!("{}", completion_order);
}
