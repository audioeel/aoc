use std::str::FromStr;
use std::slice::Iter;
use std::fs;

enum Input {
    Calories(u32),
    Separator
}

impl Input {
    fn is_separator(&self) -> bool {
        match *self {
            Input::Calories(_) => false,
            Input::Separator => true,
        }
    }

    fn or_0(&self) -> u32 {
        match *self {
            Input::Calories(num) => num,
            Input::Separator => 0,
        }
    }
}

fn count_calories(calories: Iter<Input>) -> u32 {
    calories.map(|calorie| calorie.or_0()).sum()
}

fn parse_input(line: &str) -> Input {
    if line.is_empty() {
        Input::Separator
    } else {
        // just fail in case of unexpected input
        let num = u32::from_str(line).unwrap();
        Input::Calories(num)
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day1.txt").unwrap();

    let calories: Vec<Input> = input.lines()
        .map(|line| parse_input(line))
        .collect();

    let calories_by_elf: Vec<&[Input]> = calories
        .split(|input| input.is_separator())
        .collect();

    let mut calorie_count_by_elf: Vec<u32> = calories_by_elf.iter()
       .map(|elf_calories| count_calories(elf_calories.iter()))
       .collect();

    calorie_count_by_elf.sort_by(|x, y| y.cmp(x));
    
    let max_calories = calorie_count_by_elf[0];
    
    let top3_calories = calorie_count_by_elf.iter().take(3).sum::<u32>();

    println!("{}", max_calories);
    println!("{}", top3_calories);
}

