#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from_line(line: &str) -> Self {
        let mut split = line.split(", ");
        let x = split
            .next()
            .expect("Point format incorrect!")
            .parse()
            .unwrap();
        let y = split
            .next()
            .expect("Point format incorrect!")
            .parse()
            .unwrap();

        Self { x, y }
    }

    pub fn dist_from(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}
