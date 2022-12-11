use std::fs;
use std::collections::HashSet;
use std::ops;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl ops::Sub<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::SubAssign for Coordinates {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { x: self.x - rhs.x, y: self.y - rhs.y };
    }
}

fn parse_move(line: &str) -> Move {
    let mut iter = line.split(' ');
    let direction = iter.next().unwrap();
    let steps = i32::from_str_radix(iter.next().unwrap(), 10).unwrap();
    match direction {
        "R" => Move { direction: Direction::Right, steps: steps, },
        "L" => Move { direction: Direction::Left, steps: steps, },
        "U" => Move { direction: Direction::Up, steps: steps, },
        "D" => Move { direction: Direction::Down, steps: steps, },
        _   => panic!("unrecognized direction: {:?}", direction),
    }
}

fn compute_head_step(direction: &Direction) -> Coordinates {
    match *direction {
        Direction::Right => Coordinates { x: 1, y: 0 },
        Direction::Left => Coordinates { x: -1, y: 0 },
        Direction::Up => Coordinates { x: 0, y: 1 },
        Direction::Down => Coordinates { x: 0, y: -1 },
    }
}

fn compute_knot_step(delta: Coordinates) -> Coordinates {
    let x_abs = delta.x.abs();
    let y_abs = delta.y.abs();
    if x_abs < 2 && y_abs < 2 {
        Coordinates { x: 0, y: 0 }
    } else if x_abs == 2 && y_abs <= 1 {
        Coordinates { x: delta.x / 2, y: delta.y }
    } else if y_abs == 2 && x_abs <= 1 {
        Coordinates { x: delta.x, y: delta.y / 2 }
    } else if y_abs == 2 && x_abs == 2 {
        Coordinates { x: delta.x / 2, y: delta.y / 2 }
    } else {
        panic!("unexpected delta: {:?}", delta);
    }
}

fn n_knot_rope(moves: &Vec<Move>, n: usize) -> HashSet<Coordinates> {
    let mut rope = vec![Coordinates { x: 0, y: 0 }; n];
    let mut positions = HashSet::new();
    positions.insert(rope[n-1]);

    for mov in moves {
        for _ in 0..mov.steps {
            rope[0] += compute_head_step(&mov.direction);
            for i in 1..n {
                let delta = rope[i-1] - rope[i];
                let knot_step = compute_knot_step(delta);
                rope[i] += knot_step;
            }
            positions.insert(rope[n-1]);
        }
    }

    positions
}

pub fn solve() {
    let input = fs::read_to_string("resources/day9.txt").unwrap();
    let moves = input.lines().map(parse_move).collect::<Vec<Move>>();
    let tail_positions = n_knot_rope(&moves, 2);
    println!("{}", tail_positions.len());

    let tail_positions = n_knot_rope(&moves, 10);
    println!("{}", tail_positions.len());
}
