use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum SedimentType {
    Rock,
    Sand,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Sediment {
    coords: Coordinates,
    sediment_type: SedimentType,
}

fn parse_rock_coords(coords: &str) -> Coordinates {
    let mut iter = coords.split(',');
    let x = i32::from_str_radix(iter.next().unwrap(), 10).unwrap();
    let y = i32::from_str_radix(iter.next().unwrap(), 10).unwrap();
    Coordinates { x: x, y: y } 
}

fn gen_rock_range(p1: &Coordinates, p2: &Coordinates) -> Vec<Sediment> {
    let x1 = if p1.x < p2.x { p1.x } else { p2.x };
    let x2 = if p1.x < p2.x { p2.x } else { p1.x };
    let y1 = if p1.y < p2.y { p1.y } else { p2.y };
    let y2 = if p1.y < p2.y { p2.y } else { p1.y };
    let mut rocks = vec![];
    for i in x1..=x2 {
        for j in y1..=y2 {
            rocks.push(Sediment { 
                coords: Coordinates { x: i, y: j },
                sediment_type: SedimentType::Rock
            });
        }
    }
    rocks
}

fn parse_rock_range(range: &str) -> Vec<Sediment> {
    range.split(" -> ")
        .map(parse_rock_coords)
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|r| gen_rock_range(&r[0], &r[1]))
        .collect()
}

fn count_sand<F>(rocks: &Vec<Sediment>, source: &Coordinates, place_sand: F) -> (Vec<Sediment>, u32) where
    F: Fn(&HashSet<Coordinates>, &Coordinates) -> (bool, Option<Sediment>) {
    let mut occupied: HashSet<_> = rocks.iter().map(|r| r.coords.clone()).collect();
    let mut sediments = rocks.clone();
    let mut sand: u32 = 0;

    loop {
        let (stop, sediment) = place_sand(&occupied, &source);
        match sediment {
            Some(latest) => {
                sand += 1;
                occupied.insert(latest.coords.clone());
                sediments.push(latest);
            },
            None => {},
        }
        if stop { break; }
    }

    (sediments, sand)
}

fn bounded_place_sand(occupied: &HashSet<Coordinates>, source: &Coordinates,
                      upper_left: &Coordinates, lower_right: &Coordinates) -> (bool, Option<Sediment>) {
    let mut current = Coordinates { x: source.x, y: source.y + 1 };
    while current.x > upper_left.x && current.y > upper_left.y 
        && current.x < lower_right.x && current.y < lower_right.y {
        // down
        let next = Coordinates { x: current.x, y: current.y + 1 };
        if !occupied.contains(&next) {
            current = next;
            continue;
        }
        // diagonal left
        let next = Coordinates { x: current.x - 1, y: current.y + 1 };
        if !occupied.contains(&next) {
            current = next;
            continue;
        }
        // diagonal right
        let next = Coordinates { x: current.x + 1, y: current.y + 1 };
        if !occupied.contains(&next) {
            current = next;
            continue;
        }
        // otherwise stay in place
        return (false, Some(Sediment { coords: current, sediment_type: SedimentType::Sand }));
    }

    (true, None)
}

fn floored_place_sand(occupied: &HashSet<Coordinates>, source: &Coordinates,
                      floor: i32) -> (bool, Option<Sediment>) {
    let mut current = Coordinates { x: source.x, y: source.y };
    loop {
        // down
        let next = Coordinates { x: current.x, y: current.y + 1 };
        if !occupied.contains(&next) && next.y < floor {
            current = next;
            continue;
        }
        // diagonal left
        let next = Coordinates { x: current.x - 1, y: current.y + 1 };
        if !occupied.contains(&next) && next.y < floor {
            current = next;
            continue;
        }
        // diagonal right
        let next = Coordinates { x: current.x + 1, y: current.y + 1 };
        if !occupied.contains(&next) && next.y < floor {
            current = next;
            continue;
        }

        return (&current == source, Some(Sediment { coords: current, sediment_type: SedimentType::Sand }));
    }
}


fn cave_slice(start: &Coordinates, end: &Coordinates, source: &Coordinates,
              sediments: &Vec<Sediment>) -> String {
    let mut slice = String::new();
    let map: HashMap<_, _> = sediments.iter()
        .map(|sediment| (&sediment.coords, &sediment.sediment_type))
        .collect();
    for j in start.y..=end.y {
        for i in start.x..=end.x {
            let coords = Coordinates { x: i, y: j };
            if source == &coords && !map.contains_key(&source) {
                slice.push('+');
                continue;
            }
            match map.get(&coords) {
                Some(SedimentType::Rock) => slice.push('#'),
                Some(SedimentType::Sand) => slice.push('o'),
                _ => slice.push('.'),
            }
        }
        slice.push('\n');
    }
    slice
}

pub fn solve() {
    let input = fs::read_to_string("resources/day14ex.txt").unwrap();
    let rocks: Vec<Sediment> = input.lines().flat_map(|r| parse_rock_range(r).into_iter()).collect();

    let source = Coordinates { x: 500, y: 0 };
    
    let mut positions: Vec<_> = rocks.iter().map(|r| &r.coords).collect();
    positions.push(&source);

    let lowest_point = positions.iter().map(|c| c.y).max().unwrap();

    let upper_left = Coordinates {
        x: positions.iter().map(|c| c.x).min().unwrap(),
        y: positions.iter().map(|c| c.y).min().unwrap(),
    };
    let lower_right = Coordinates {
        x: positions.iter().map(|c| c.x).max().unwrap(),
        y: lowest_point,
    };

    let start = Coordinates {
        x: upper_left.x - 10,
        y: upper_left.y,
    };
    let end = Coordinates {
        x: lower_right.x + 10,
        y: lower_right.y + 2,
    };

    let (sediments, sand_count) = count_sand(&rocks, &source,
                                  move |occupied, source| bounded_place_sand(occupied, source, &upper_left, &lower_right));
    println!("{}", sand_count);
    println!("{}", cave_slice(&start, &end, &source, &sediments));


    let floor = lowest_point + 2;
    let (sediments, sand_count) = count_sand(&rocks, &source,
                                  move |occupied, source| floored_place_sand(occupied, source, floor));
    println!("{}", sand_count);
    println!("{}", cave_slice(&start, &end, &source, &sediments));
}
