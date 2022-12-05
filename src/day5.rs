use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    amount: u8,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;

pub fn solve() {
    let input = fs::read_to_string("resources/day5.txt").unwrap();
    
    let mut stacks: Stacks = vec![];    
    let mut nstacks: usize = 0;
    let mut rinstructions = vec![];
    for line in input.lines().rev() {
        // crate
        let mut words = line.split_whitespace();
        let first = words.next(); 
        match first {
            Some("move") => { 
                let amount = u8::from_str(words.next().unwrap()).unwrap();
                words.next(); // 'from' keyword
                let from = usize::from_str(words.next().unwrap()).unwrap();
                words.next(); // 'to' keyword
                let to = usize::from_str(words.next().unwrap()).unwrap();

                rinstructions.push(Instruction { amount: amount, from: from, to: to });
            }
            Some("1") => {
                nstacks = usize::from_str(words.last().unwrap()).unwrap();
                for _ in 0..nstacks { stacks.push(vec![]); }
            }
            // empty separator line
            None => {}
            // crate row
            _ => {
                let mut iter = line.chars();
                for istack in 0..nstacks {
                    iter.next(); // '[' or empty space
                    match iter.next() {
                        Some(' ') => {},
                        Some(c) => { stacks[istack].push(c); },
                        _ => { }
                    }
                    iter.next(); // ']' or empty space
                    iter.next(); // ' ' separator or None
                }
            }
        }
    }

    for instruction in rinstructions.iter().rev() {
        println!("{:?}", instruction);
        let from = &mut stacks[instruction.from - 1];
        let mut buffer: Vec<char> = vec![]; 
        for _ in  0..instruction.amount {
            match from.pop() {
                Some(c) => { buffer.push(c); }
                _ => {}
            }
        }

        let to = &mut stacks[instruction.to - 1];
        for c in buffer {
            to.push(c);
        }
    }

    for stack in stacks {
        match stack.last() {
            Some(c) => { print!("{c}"); }
            _ => {}
        }
    }
    println!("");
}
