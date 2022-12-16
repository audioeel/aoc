use std::fs;
use std::iter;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::collections::HashSet;
use bit_set::BitSet;
use rayon::prelude::*;

use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
struct Valve {
    flow: u32,
    tunnels: Vec<usize>,
}

struct ParsedElement {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
struct State {
    position: usize,
    flow: u32,
    opened: BitSet,
}

#[derive(Debug, Clone)]
struct ElephantState {
    my_position: usize,
    elephant_position: usize,
    flow: u32,
    opened: BitSet,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
           && self.flow == other.flow
           && self.opened.len() == other.opened.len()
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
       self.position.cmp(&other.position)
           .then_with(|| self.opened.len().cmp(&other.opened.len()))
           .then_with(|| self.flow.cmp(&other.flow))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //self.position.hash(state);
        self.opened.len().hash(state);
        self.flow.hash(state);
    }
}

impl PartialEq for ElephantState {
    fn eq(&self, other: &Self) -> bool {
        (self.my_position == other.my_position || self.my_position == other.elephant_position)
            && (self.elephant_position == other.elephant_position || self.elephant_position == other.my_position)
            && self.opened.len() == other.opened.len()
            && self.flow == other.flow
    }
}

impl Eq for ElephantState {}

impl Ord for ElephantState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flow.cmp(&other.flow)
    }
}

impl PartialOrd for ElephantState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ElephantState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.flow.hash(state);
    }
}


fn parse(input: &str) -> Vec<Valve> {
    let rg = Regex::new(
        r"Valve (?P<name>[A-Z][A-Z]) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<tunnels>[A-Z][A-Z](, [A-Z][A-Z])*)").unwrap();

    let parsed: Vec<_> = input.lines()
        .map(|line| rg.captures(line).unwrap())
        .map(|cap| ParsedElement {
            name: String::from(&cap["name"]),
            flow: u32::from_str_radix(&cap["flow"], 10).unwrap(),
            tunnels: cap["tunnels"].split(", ").map(|s| String::from(s)).collect(),
        })
        .sorted_by(|v1, v2| v1.name.cmp(&v2.name))
        .collect();

    let names: Vec<String> = parsed.iter().map(|p| p.name.clone()).collect();

    let valves = parsed.iter()
        .map(|node| Valve {
            flow: node.flow,
            tunnels: node.tunnels.iter().map(|t| names.iter().position(|s| s == t).unwrap()).collect(),
        })
        .collect();

    valves
}

fn evolve(state: &State, valves: &Vec<Valve>, remaining: u32) -> Vec<State> {
    let mut next = vec![];
    let valve_flow = valves[state.position].flow;
    if !state.opened.contains(state.position) && valve_flow > 0 {
        next.push(State {
            position: state.position,
            flow: state.flow + valve_flow * remaining,
            opened: state.opened.iter().chain(iter::once(state.position)).collect(),
        });
    }

    for valve in &valves[state.position].tunnels {
        next.push(State {
            position: *valve,
            flow: state.flow,
            opened: state.opened.clone(),
        });
    }

    next
}

fn evolve_elephant(state: &ElephantState, valves: &Vec<Valve>, remaining: u32) -> Vec<ElephantState> {
    if valves.len() == state.opened.len() {
        return vec![];
    }

    let mut next = vec![];
    let my_valve_flow = valves[state.my_position].flow;
    let elephant_valve_flow = valves[state.elephant_position].flow;
    let open_my_valve = !state.opened.contains(state.my_position) && my_valve_flow > 0;
    let open_elephant_valve = !state.opened.contains(state.elephant_position) && elephant_valve_flow > 0 && state.my_position != state.elephant_position;

    if open_my_valve && open_elephant_valve {
        next.push(ElephantState {
            my_position: state.my_position,
            elephant_position: state.elephant_position,
            flow: state.flow + my_valve_flow * remaining + elephant_valve_flow * remaining,
            opened: state.opened.iter().chain(vec![state.my_position, state.elephant_position].into_iter()).collect(),
        });
    } else if open_my_valve && !open_elephant_valve {
        for elephant_valve in &valves[state.elephant_position].tunnels {
            if !state.opened.contains(*elephant_valve) && *elephant_valve != state.my_position {
                next.push(ElephantState {
                    my_position: state.my_position,
                    elephant_position: *elephant_valve,
                    flow: state.flow + my_valve_flow * remaining,
                    opened: state.opened.iter().chain(iter::once(state.my_position)).collect(),
                });
            }
        }
    } else if !open_my_valve && open_elephant_valve {
        for my_valve in &valves[state.my_position].tunnels {
            if !state.opened.contains(*my_valve) && *my_valve != state.elephant_position {
                next.push(ElephantState {
                    my_position: *my_valve,
                    elephant_position: state.elephant_position,
                    flow: state.flow + elephant_valve_flow * remaining,
                    opened: state.opened.iter().chain(iter::once(state.elephant_position)).collect(),
                });
            }
        }
    }
    // don't open any valves, just change positions for both the elephant and me
    valves[state.my_position].tunnels.iter()
        .cartesian_product(valves[state.elephant_position].tunnels.iter())
        .unique()
        //.filter(|(my, eleph)| my != eleph)
        //.sorted_by(|x, y| (x.0 + x.1).cmp(&(y.0 + y.1)).then_with(|| (x.0 - x.1).cmp(&(y.0 - y.1))))
        //.dedup_by(|x, y| (x.0 == y.0 && x.1 == y.1) || (x.0 == y.1 && x.1 == y.0))
        .for_each(|(my, eleph)| {
            next.push(ElephantState {
                    my_position: *my,
                    elephant_position: *eleph,
                    flow: state.flow,
                    opened: state.opened.clone(),
                });
        });
    next
}


fn max_pressure_alone(valves: &Vec<Valve>) -> State {
    let minutes = 30;

    let mut current = HashSet::new();
    current.insert(State {
        position: 0,
        flow: 0,
        opened: BitSet::from_bytes(&[0; 7]),
    });    

    for minute in 1..=minutes {
        println!("minute: {}, num_states: {}", minute, current.len());
        if minute <= 5 {
            println!("states: {:?}", current);
        }

        current = current.into_par_iter()
            .flat_map(|state| evolve(&state, &valves, minutes - minute))
            .collect();
    }

    current.into_par_iter()
        .max_by(|x,y| x.flow.cmp(&y.flow))
        .unwrap()
}

fn max_pressure_with_elephant(valves: &Vec<Valve>) -> ElephantState {
    let minutes = 26;

    let mut current = HashSet::new();
    current.insert(ElephantState {
        my_position: 0,
        elephant_position: 0,
        flow: 0,
        opened: BitSet::from_bytes(&[0; 7]),
    });    

    for minute in 1..=minutes {
        println!("minute: {}, num_states: {}", minute, current.len());
        if minute <= 5 {
            println!("states: {:?}", current);
        }

        current = current.into_par_iter()
            .flat_map(|state| evolve_elephant(&state, &valves, minutes - minute))
            .collect();
    }

    current.into_par_iter()
        .max_by(|x,y| x.flow.cmp(&y.flow))
        .unwrap()
}

pub fn solve() {
    let input = fs::read_to_string("resources/day16.txt").unwrap();
    let valves = parse(&input);

    println!("{:?}", max_pressure_with_elephant(&valves));
}
