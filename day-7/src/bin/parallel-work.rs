use std::io::{prelude::*, BufReader};

use day_7::{Task, TaskDependency};
use rayon::prelude::*;

fn main() {
    let matches = clap::App::new("parallel-work")
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

        tasks[dep.target as usize].remaining_time = 60u8 + dep.target + 1;
        tasks[dep.dependency as usize].remaining_time = 60u8 + dep.dependency + 1;
    }

    // sort and dedup dependencies and dependents in task list
    for task in &mut tasks {
        task.dependencies.sort();
        task.dependencies.dedup();

        task.dependents.sort();
        task.dependents.dedup();
    }

    let mut num_completed = tasks.iter().filter(|t| t.remaining_time == 0).count();

    let mut completion_time: u32 = 0;

    let mut in_progress = vec![];
    let mut in_progress_removals = vec![];
    let mut completed = vec![];
    let mut removals = vec![];

    while num_completed != 26 {
        in_progress_removals.clear();
        completed.clear();
        removals.clear();

        let start_in_progress_length = in_progress.len();

        // Mark prog
        let new_in_progress = tasks
            .iter_mut()
            .enumerate()
            .filter(|(_, t)| !t.in_progress)
            .filter(|(_, t)| t.remaining_time > 0)
            .filter(|(_, t)| t.dependencies.is_empty())
            .map(|(i, _)| i)
            .take(5 - start_in_progress_length);

        // Get some more work to do
        in_progress.extend(new_in_progress);

        // Mark new tasks as in_progress
        for task_idx in &in_progress[start_in_progress_length..] {
            tasks[*task_idx as usize].in_progress = true;
        }

        println!("Startings tasks: {:?}", in_progress);

        assert!(!in_progress.is_empty());

        // figure out which task is closest to being completed
        let time_to_run = in_progress
            .iter()
            .cloned()
            .map(|task_idx| tasks[task_idx].remaining_time)
            .min()
            .unwrap();

        completion_time += time_to_run as u32;

        // decrease remaining_time on all in-progress tasks and track completed
        for (i, task_idx) in in_progress.iter().enumerate() {
            tasks[*task_idx as usize].remaining_time -= time_to_run;

            if tasks[*task_idx as usize].remaining_time == 0 {
                completed.push(*task_idx);
                in_progress_removals.push(i);
            }
        }

        // Remove from in_progress in reverse
        for to_remove in in_progress_removals.iter().rev() {
            in_progress.remove(*to_remove);
        }

        num_completed += completed.len();

        // Remove completed tasks
        for task_idx in &completed {
            println!("{} completing...", *task_idx);

            for dependent in &tasks[*task_idx as usize].dependents {
                let remove_at = tasks[*dependent as usize]
                    .dependencies
                    .binary_search(&(*task_idx as u8))
                    .expect("Task dependency wasn't recorded");
                removals.push((*dependent, remove_at));
            }

            println!("{} completed!", *task_idx);
        }

        for removal in &removals {
            tasks[removal.0 as usize].dependencies.remove(removal.1);
        }
    }

    println!("{}", completion_time);
}
