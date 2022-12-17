use std::fs;
use bit_set::BitSet;
use itertools::Itertools;

use regex::Regex;

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

fn parse(input: &str) -> (Vec<String>, Vec<Valve>) {
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

    (names, valves)
}

fn make_flows_over_time(valves: &Vec<Valve>, minutes: usize) -> Vec<Vec<i32>> {
   (1..=minutes)
       .map(|minute| valves.iter()
            .map(|valve| valve.flow as i32 * (minutes - minute) as i32)
            .collect())
       .collect()
}

fn make_sources_by_destination(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let n = valves.len();
    let mut sources = vec![vec![]; valves.len()];
    for i in 0..n {
        for &j in &valves[i].tunnels {
            sources[j].push(i);
        }
    }
    sources
}

fn compute_cumulative_flows(valves: &Vec<Valve>, flows: &Vec<Vec<i32>>, minutes: usize) -> Vec<Vec<i32>> {
    let n = valves.len();

    // cflow is cumulative flow after the ith minute
    let mut cflow: Vec<Vec<i32>> = vec![vec![-1; n]; minutes];

    // opened is opened valves after the ith minute
    let mut opened = vec![vec![BitSet::from_bytes(&[0; 8]); n]; minutes];

    cflow[0][0] = flows[0][0];
    opened[0][0].insert(0);
    for &j in &valves[0].tunnels {
        cflow[0][j] = 0;
    }

    for i in 1..minutes {
        // j is our starting point: we'll see which paths are attainable from j during minute i
        for j in 0..n {
            // a cflow of -1 indicates that there's no path from j to any of the valves at minute i
            if cflow[i-1][j] < 0 {
                continue;
            }
            // open a valve if it's not already done and stay in place
            let valve_j_already_opened = opened[i-1][j].contains(j);
            let flow_at_j = cflow[i-1][j] + (if valve_j_already_opened { 0 } else { flows[i][j] });
            if flow_at_j >= cflow[i][j] {
                cflow[i][j] = flow_at_j;
                opened[i][j] = opened[i-1][j].clone();
                if !valve_j_already_opened {
                    opened[i][j].insert(j);
                }
            }
            // or try moving to other places, if moving from j to k maximizes flow
            for &k in &valves[j].tunnels {
                if cflow[i-1][j] >= cflow[i][k] {
                    cflow[i][k] = cflow[i-1][j];
                    opened[i][k] = opened[i-1][j].clone();
                }
            }
        }
    }

    cflow
}

fn make_optimal_path(cflow: &Vec<Vec<i32>>, flows: &Vec<Vec<i32>>, sources: &Vec<Vec<usize>>) -> Vec<usize> {
    let minutes = cflow.len();
    let n = sources.len();

    let (idx, max) = cflow[minutes-1].iter().enumerate().max_by(| (_, v1), (_, v2) | v1.cmp(v2)).unwrap();

    let mut optimal = vec![idx];
    let mut current = idx;
    for minute in (1..minutes).rev() {
        if cflow[minute][current] == cflow[minute-1][current] + flows[minute][current] {
            optimal.push(current);
            continue;
        }
        for &j in &sources[current] {
            if cflow[minute][current] == cflow[minute-1][j] {
                current = j;
                optimal.push(j);
                break;
            }
        }
    }

    optimal.into_iter().rev().collect()
}

fn display_flows(names: &Vec<String>, cumulative_flows: &Vec<Vec<i32>>) {
    let minutes = cumulative_flows.len();
    let n = names.len();
    print!("     ");
    for i in 0..n {
        print!("{header:>5}", header=names[i]);
    }
    print!("\n");
    for i in 0..minutes {
        print!("{number:>3}: ", number=i+1);
        for j in 0..n {
            print!("{number:>5}", number=cumulative_flows[i][j]);
        }
        print!("\n");
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day16.txt").unwrap();
    let (names, valves) = parse(&input);

    let n: usize = valves.len();
    let minutes: usize = 30;

    let flows_over_time = make_flows_over_time(&valves, minutes);

    let cumulative_flows = compute_cumulative_flows(&valves, &flows_over_time, minutes);
    println!("cumulative flows:");
    display_flows(&names, &cumulative_flows);

    println!("flows over time:");
    display_flows(&names, &flows_over_time);

    let sources = make_sources_by_destination(&valves);
    let optimal = make_optimal_path(&cumulative_flows, &flows_over_time, &sources);

    println!("max flow: {}", cumulative_flows[minutes-1][*optimal.last().unwrap()]);
    println!("optimal path (len={}): {}", optimal.len(), optimal.iter().map(|idx| &names[*idx]).join(" -> ")); 
}
