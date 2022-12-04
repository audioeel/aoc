use std::fs;
use std::str::FromStr;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(string: &str) -> Range {
        let mut iter = string.split('-');
        let s = u32::from_str(iter.next().unwrap()).unwrap();
        let e = u32::from_str(iter.next().unwrap()).unwrap();
        if s > e {
            panic!("unexpected range: {s} > {e}");
        }
        Range { start: s, end: e }
    }
}

struct Assignment {
    r1: Range,
    r2: Range,
}

impl Assignment {
    fn new(line: &str) -> Assignment {
        let mut iter = line.split(',');
        let r1 = Range::new(iter.next().unwrap());
        let r2 = Range::new(iter.next().unwrap());
        Assignment { r1: r1, r2: r2 }
    }

    fn full_overlap(&self) -> bool {
        let r1_contains_r2 = self.r1.start <= self.r2.start && self.r1.end >= self.r2.end;
        let r2_contains_r1 = self.r2.start <= self.r1.start && self.r2.end >= self.r1.end;
        r1_contains_r2 || r2_contains_r1
    }

    fn no_overlap(&self) -> bool {
        let r1_before_r2 = self.r1.end < self.r2.start;
        let r2_before_r1 = self.r2.end < self.r1.start;
        r1_before_r2 || r2_before_r1
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day4.txt").unwrap();

    let assignments: Vec<Assignment> = input.lines().map(Assignment::new).collect();

    let fully_overlapping: Vec<&Assignment> = assignments
        .iter()
        .filter(|assignment| assignment.full_overlap())
        .collect();

    println!("{}", fully_overlapping.len());

    let overlapping: Vec<&Assignment> = assignments
        .iter()
        .filter(|assignment| !assignment.no_overlap())
        .collect();

    println!("{}", overlapping.len());
}
