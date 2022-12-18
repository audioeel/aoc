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

fn compute_cumulative_flows(valves: &Vec<Valve>, flows: &Vec<Vec<i32>>, minutes: usize) -> Vec<Vec<Vec<i32>>> {
    let n = valves.len();

    // cflow is cumulative flow after the ith minute
    let mut cflow: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; n]; n]; minutes];

    // opened is opened valves after the ith minute
    let mut opened = vec![vec![vec![BitSet::from_bytes(&[0; 8]); n]; n]; minutes];

    cflow[0][0][0] = flows[0][0];
    opened[0][0][0].insert(0);
    for &j in &valves[0].tunnels {
        cflow[0][0][j] = 0;
    }

    for m in 1..minutes {
        // i is the starting point
        for i in 0..n {
            // signals that this starting point has not been traversed yet
            if cflow[m-1][i][i] == -1 {
                continue;
            }

            if cflow[m-1][i][i] > cflow[m][i][i] {
                cflow[m][i][i] = cflow[m-1][i][i];
                opened[m][i][i] = opened[m-1][i][i].clone();
            }

            // ending points from (i-1)th minute
            for &j in &valves[i].tunnels {
                if cflow[m-1][i][i] > cflow[m][i][j] {
                    cflow[m][i][j] = cflow[m-1][i][i];
                    opened[m][i][j] = opened[m-1][i][i].clone();
                }

                let valve_j_already_opened = opened[m-1][i][j].contains(j);
                let flow_at_j = cflow[m-1][i][j] + (if valve_j_already_opened { 0 } else { flows[m][j] });
                if flow_at_j > cflow[m][j][j] {
                    cflow[m][j][j] = flow_at_j;
                    opened[m][j][j] = opened[m-1][i][j].clone();
                    if !valve_j_already_opened { opened[m][j][j].insert(j); }
                }
                for &k in &valves[j].tunnels {
                    if cflow[m-1][i][j] > cflow[m][j][k] {
                        cflow[m][j][k] = cflow[m-1][i][j];
                        opened[m][j][k] = opened[m-1][i][j].clone();
                    }
                }
            }
        }
    }

    cflow
}

fn make_optimal_path(cflow: &Vec<Vec<Vec<i32>>>, flows: &Vec<Vec<i32>>,
                     sources: &Vec<Vec<usize>>) -> Vec<usize> {
    let minutes = cflow.len();

    let ((start, end), max_flow) = cflow[minutes-1].iter().enumerate()
        .map(|(i, row)| {
            let (j, v) = row.iter().enumerate()
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .unwrap();
            ((i, j), v)
        })
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();


    for row in &cflow[minutes-3] {
        println!("{:?}", row);
    }

    let mut optimal = vec![];
    let mut cstart = start;
    let mut cend = end;
    for minute in (1..minutes).rev() {
        if cflow[minute][cstart][cend] == cflow[minute-1][cstart][cend] + flows[minute][cstart] {
            optimal.push(cstart);
            optimal.push(cend);
            continue;
        }

        for &j in &sources[cstart] {
            if cflow[minute][cstart][cend] == cflow[minute-1][j][cstart] {
                cend = cstart;
                cstart = j;
                optimal.push(cstart);
                break;
            }
        }
    }

    optimal.into_iter().rev().collect()
}

pub fn solve() {
    let input = fs::read_to_string("resources/day16ex.txt").unwrap();
    let (names, valves) = parse(&input);

    let n: usize = valves.len();
    let minutes: usize = 30;

    let flows_over_time = make_flows_over_time(&valves, minutes);

    let cumulative_flows = compute_cumulative_flows(&valves, &flows_over_time, minutes);

    let sources = make_sources_by_destination(&valves);
    let optimal = make_optimal_path(&cumulative_flows, &flows_over_time, &sources);

    println!("optimal path (len={}): {}", optimal.len(), optimal.iter().map(|idx| &names[*idx]).join(" -> "));
}
