use std::fs;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::{min, max};

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct XRange {
    lo: i32,
    hi: i32,
}

impl XRange {
    fn contains(&self, x: i32) -> bool {
        x >= self.lo && x <= self.hi
    }

    fn len(&self) -> usize {
        (self.hi - self.lo + 1) as usize
    }

    fn merge(r1: &XRange, r2: &XRange) -> Vec<XRange> {
        // r1 contains r2
        if r1.lo <= r2.lo && r1.hi >= r2.hi {
            return vec![r1.clone()];
        }

        // overlapping
        if r1.hi >= r2.lo && r2.hi >= r1.hi {
            return vec![ XRange { lo: r1.lo, hi: r2.hi }];
        }

        // touching
        if r2.lo - 1 == r1.hi {
            return vec![ XRange { lo: r1.lo, hi: r2.hi }];
        }

        // disjoint
        if r2.lo > r1.hi {
            return vec![r1.clone(), r2.clone()];
        }

        panic!("unexpected intervals: r1={:?}, r2={:?}", r1, r2);
    }
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Coordinates,
    beacon: Coordinates,
}

fn manhattan(p1: &Coordinates, p2: &Coordinates) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn gen_xrange_for_y(coords: &Coordinates, target_y: i32, max_distance: i32) -> XRange {
    let distance_from_target = (target_y - coords.y).abs();
    let allowable_x_distance = (max_distance - distance_from_target).abs();
    XRange {
        lo: max(0, coords.x - allowable_x_distance),
        hi: min(4_000_000, coords.x + allowable_x_distance),
    }
}

fn gen_xrange(coords: &Coordinates, max_distance: i32) -> impl Iterator<Item = (i32, Vec<XRange>)> + '_ {
    let lo = max(0, coords.y - max_distance);
    let hi = min(4_000_000, coords.y + max_distance);
    (lo..=hi)
        .map(move |y| (y, vec![gen_xrange_for_y(&coords, y, max_distance)]))
}

// fn count_empty_on_row(pairs: &Vec<SensorBeacon>, y: i32) -> usize {
//     let covered: XRange = pairs.iter()
//         .map(|sb| gen_xrange_for_y(&sb.sensor, y, manhattan(&sb.sensor, &sb.beacon)))
//         .reduce(|accum, range| XRange::merge(&accum, &range))
//         .unwrap();
// 
//     let nbeacons = pairs.iter()
//         .map(|p| &p.beacon)
//         .unique()
//         .filter(|b| b.y == y && covered.contains(b.x))
//         .count();
// 
//     covered.len() - nbeacons
// }

fn find_distress_beacon(pairs: &Vec<SensorBeacon>) -> Vec<(i32, Vec<XRange>)> {
    pairs.iter()
        .flat_map(|sb| gen_xrange(&sb.sensor, manhattan(&sb.sensor, &sb.beacon)))
        .sorted_by(|(y1, _), (y2, _)| y1.cmp(&y2))
        .coalesce(|(y1, intervals1), (y2, intervals2)| if y1 == y2 {
            Ok((y1, intervals1.into_iter().chain(intervals2.into_iter())
                .sorted_by(|interval1, interval2| interval1.lo.cmp(&interval2.lo))
                .collect()))
        } else {
            Err(((y1, intervals1), (y2, intervals2)))
        })
        .filter(|(_, intervals)| {
            let mut accum = intervals[0].clone();
            for interval in intervals.iter().skip(1) {
                let mut merged = XRange::merge(&accum, &interval);
                if merged.len() == 1 {
                    accum = merged[0].clone();
                } else {
                    return true;
                }
            }
            return false;
        })
        .map(|(y, intervals)| {
            let mut accum = intervals[0].clone();
            for interval in intervals.iter().skip(1) {
                let mut merged = XRange::merge(&accum, &interval);
                if merged.len() == 1 {
                    accum = merged[0].clone();
                } else {
                    return (y, merged);
                }
            }
            return (y, vec![accum]);

        })
        .collect()
}

fn parse(input: &str) -> Vec<SensorBeacon> {
    let rg = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();
    input.lines()
        .map(|line| rg.captures(line).unwrap())
        .map(|cap| SensorBeacon {
            sensor: Coordinates {
                x: i32::from_str_radix(&cap["sensor_x"], 10).unwrap(),
                y: i32::from_str_radix(&cap["sensor_y"], 10).unwrap(),
            },
            beacon: Coordinates {
                x: i32::from_str_radix(&cap["beacon_x"], 10).unwrap(),
                y: i32::from_str_radix(&cap["beacon_y"], 10).unwrap(),
            }
        }).collect()
}

pub fn solve() {
    let input = fs::read_to_string("resources/day15.txt").unwrap();
    let y = 2_000_000;
    // let y = 10;

    let pairs = parse(&input);
    
    // println!("{}", count_empty_on_row(&pairs, y)); 
    let distress_coords = find_distress_beacon(&pairs);
    println!("{:?}", distress_coords);
}
