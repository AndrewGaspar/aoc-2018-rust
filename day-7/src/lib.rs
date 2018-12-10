lazy_static::lazy_static! {
    static ref LINE_REGEX: regex::Regex =
        regex::Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
            .unwrap();
}

#[derive(Copy, Clone, Debug)]
pub struct TaskDependency {
    pub dependency: u8,
    pub target: u8,
}

impl TaskDependency {
    pub fn from_line(line: &str) -> Self {
        let captures = LINE_REGEX.captures(line).expect("Input is malformed!");

        Self {
            dependency: captures[1].as_bytes()[0] - b'A',
            target: captures[2].as_bytes()[0] - b'A',
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Task {
    pub dependencies: Vec<u8>,
    pub dependents: Vec<u8>,
    pub incomplete: bool,
    pub in_progress: bool,
    pub remaining_time: u8,
}
