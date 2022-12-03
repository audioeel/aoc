use std::fs;

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Loss = -1,
    Draw = 0,
    Win = 1,
}

fn parse_them(ch: char) -> Choice {
    match ch {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => panic!("unexpected choice for player 1"),
    }
}

fn parse_us(ch: char) -> Choice {
    match ch {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scissors,
        _ => panic!("unexpected choice for player 2"),
    }
}

fn parse_outcome(ch: char) -> Outcome {
    match ch {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("unexpected choice for outcome"),
    }
}

fn score_us_vs_them(us: i32, them: i32) -> i32 {
    let diff = us - them;

    us // choice points
        + 3 * (diff == 0) as i32 // tie points
        + 6 * (diff == -2 || diff == 1) as i32 // victory points
}

trait Day2Input {
    fn parse(line: &str) -> Self;
    fn score(&self) -> i32;
}

#[derive(Clone, Copy)]
struct Input {
    them: Choice,
    us: Choice,
}

impl Day2Input for Input {
    fn parse(line: &str) -> Input {
        let mut input = line.chars();

        let them = parse_them(input.next().unwrap());
        assert!(input.next().unwrap() == ' ', "unexpected input");
        let us = parse_us(input.next().unwrap());

        Input { us: us, them: them }
    }

    fn score(&self) -> i32 {
        score_us_vs_them(self.us as i32, self.them as i32)
    }
}

#[derive(Clone, Copy)]
struct Input2 {
    them: Choice,
    outcome: Outcome,
}

impl Day2Input for Input2 {
    fn parse(line: &str) -> Input2 {
        let mut input = line.chars();

        let them = parse_them(input.next().unwrap());
        assert!(input.next().unwrap() == ' ', "unexpected input");
        let outcome = parse_outcome(input.next().unwrap());

        Input2 {
            outcome: outcome,
            them: them,
        }
    }

    fn score(&self) -> i32 {
        let them = self.them as i32;
        let outcome = self.outcome as i32;
        let sum = them + outcome;
        let us = if sum > 3 {
            1
        } else if sum < 1 {
            3
        } else {
            sum
        };
        score_us_vs_them(us, them)
    }
}

fn compute_total_score<I: Day2Input>(input: String) -> i32 {
    let rounds: Vec<I> = input.lines().map(I::parse).collect();

    let score_per_round: Vec<i32> = rounds.iter().map(|round| round.score()).collect();

    score_per_round.iter().sum()
}

pub fn solve() {
    let input = fs::read_to_string("resources/day2.txt").unwrap();

    println!("{}", compute_total_score::<Input>(input.clone()));

    println!("{}", compute_total_score::<Input2>(input.clone()));
}
