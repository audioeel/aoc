use std::fs;

enum Instruction {
    Noop,
    Addx(i32),
}

fn decode(line: &str) -> Instruction {
    if line.starts_with("noop") {
        Instruction::Noop
    } else if line.starts_with("addx") {
        let mut iter = line.split(' ');
        iter.next();
        let increment = i32::from_str_radix(iter.next().unwrap(), 10).unwrap();
        Instruction::Addx(increment)
    } else {
        panic!("unexpected instruction: {}", line);
    }
}

fn simulate_processor(instructions: Vec<Instruction>) -> Vec<i32> {
    let mut state: Vec<i32> = vec![];
    let mut x = 1;

    for instruction in instructions {
        state.push(x);
        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(i) => {
                state.push(x);
                x += i;
            },
        }
    }

    state
}

fn compute_signal_strength(state: &Vec<i32>) -> i32 {
    let mut signal = 0;
    for (i, &x) in state.iter().enumerate() {
        let cycle = i + 1;
        signal += if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
            x * (cycle as i32)
        } else {
            0
        };
    }
    signal
}

const NROWS: usize = 6;
const NCOLS: usize = 40;

fn scan_display(state: &Vec<i32>) -> [[char; NCOLS]; NROWS] {
    let mut display: [[char; NCOLS]; NROWS] = [['.'; NCOLS]; NROWS];
    for (c, &x) in state.iter().enumerate() {
        let row = c / NCOLS;
        let col = c - NCOLS * row;
        let sprite = [x - 1, x, x + 1];
        for pixel in sprite {
            if pixel < 0 || pixel >= NCOLS.try_into().unwrap() { continue; }
            if pixel == col.try_into().unwrap() {
                display[row][col] = '#';
            }
        }
    }
    display
}

pub fn solve() {
    let input = fs::read_to_string("resources/day10.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(decode).collect();
    let state = simulate_processor(instructions);
    let signal_strength = compute_signal_strength(&state);
    println!("{}", signal_strength);

    let display = scan_display(&state);
    for row in display {
        for pixel in row {
            print!("{}", pixel);
        }
        print!("\n");
    }
}
