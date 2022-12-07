use std::fs;


pub fn solve() {
    let input = fs::read_to_string("resources/day6.txt").unwrap();
    let letters: Vec<char> = input.chars().collect();

    let n = letters.len();
    let mut is_unique: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for i in 0..n {
        is_unique[i][i] = true;
    }

    for i in 0..n {
        for j in i+1..n {
            is_unique[i][j] = is_unique[i][j - 1] && !letters[i..j].contains(&letters[j]);
        }
    }

    let mut start_of_packet = -1;
    let mut start_of_message = -1;
    for i in 0..n {
        if start_of_packet < 0 && is_unique[i][i+3] {
            start_of_packet = (i+4) as i32;
        }

        if start_of_message < 0 && is_unique[i][i+13] {
            start_of_message = (i+14) as i32;
        }

        if start_of_packet >= 0 && start_of_message >= 0 {
            break;
        }
    }
    println!("start of packet: {}", start_of_packet);
    println!("start of message: {}", start_of_message);

}
