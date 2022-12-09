use std::fs;

fn find_marker<const W: usize>(letters: &Vec<char>) -> Option<usize> {
    let n = letters.len();
    let mut start = 0;
    for i in 1..n {
        // if within window bounds, pick up where we left off
        let bound = if start + W > i { start } else { i - W };
        start = i;
        for j in bound..i {
            if !letters[j..i].contains(&letters[i]) {
                start = j;
                break;
            }
        }
        if i - start == W {
            return Some(i);
        }
    }
    None
}

pub fn solve() {
    let input = fs::read_to_string("resources/day6.txt").unwrap();
    let letters: Vec<char> = input.chars().collect();

    println!("start of packet: {}", find_marker::<4>(&letters).unwrap());
    println!("start of message: {}", find_marker::<14>(&letters).unwrap());
}
