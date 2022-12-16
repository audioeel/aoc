use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    id: String,
    flow: u32,
    tunnels: Vec<String>,
}

fn parse(input: &str) -> Vec<Valve> {
    let rg = Regex::new(
        r"Valve (?P<name>[A-Z][A-Z]) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<tunnels>[A-Z][A-Z](, [A-Z][A-Z])*)").unwrap();

    input.lines()
        .map(|line| rg.captures(line).unwrap())
        .map(|cap| Valve {
            id: String::from(&cap["name"]),
            flow: u32::from_str_radix(&cap["flow"], 10).unwrap(),
            tunnels: cap["tunnels"].split(", ").map(|s| String::from(s)).collect(),
        }).collect()
}

pub fn solve() {
    let input = fs::read_to_string("resources/day16ex.txt").unwrap();
    let valves = parse(&input);
    println!("{:?}", valves);
}
