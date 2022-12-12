use std::fs;

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
    Double,
}

struct Monkey {
    operation: Operation,
    divisor: u64,
    if_branch: usize,
    else_branch: usize,
}

type Inventory = Vec<u64>;

fn parse(input: String) -> (Vec<Inventory>, Vec<Monkey>) {
    let mut iter = input.lines();
    let mut inventories: Vec<Inventory> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];

    loop {
        iter.next();

        let mut items_iter = iter.next().unwrap().split(": ");
        items_iter.next();
        let items: Vec<u64> = items_iter.next().unwrap().split(", ")
            .map(|item| u64::from_str_radix(item, 10).unwrap())
            .collect();

        let operation_str = iter.next().unwrap();
        let operation = if operation_str.contains('*') {
            let mut operation_iter = operation_str.split(" * ");
            operation_iter.next();
            let term_str = operation_iter.next().unwrap();
            if term_str == "old" {
                Operation::Square
            } else {
                let constant = u64::from_str_radix(term_str, 10).unwrap();
                Operation::Multiply(constant)
            }
        } else if operation_str.contains('+') {
            let mut operation_iter = operation_str.split(" + ");
            operation_iter.next();
            let term_str = operation_iter.next().unwrap();
            if term_str == "old" {
                Operation::Double
            } else {
                let constant = u64::from_str_radix(term_str, 10).unwrap();
                Operation::Add(constant)
            }
        } else {
            panic!("unhandled operation");
        };

        let test_str = iter.next().unwrap();
        let mut test_iter = test_str.split(" divisible by ");
        test_iter.next();
        let divisor = u64::from_str_radix(test_iter.next().unwrap(), 10).unwrap();

        let if_true_str = iter.next().unwrap();
        let mut if_true_iter = if_true_str.split(" throw to monkey ");
        if_true_iter.next();
        let if_branch = usize::from_str_radix(if_true_iter.next().unwrap(), 10).unwrap();

        let if_false_str = iter.next().unwrap();
        let mut if_false_iter = if_false_str.split(" throw to monkey ");
        if_false_iter.next();
        let else_branch = usize::from_str_radix(if_false_iter.next().unwrap(), 10).unwrap();

        inventories.push(items);

        monkeys.push(Monkey {
            operation: operation,
            divisor: divisor,
            if_branch: if_branch,
            else_branch: else_branch,
        });

        match iter.next() {
            Some(_) => { },
            None => { break; },
        }
    }

    (inventories, monkeys)
}

fn compute_worry(operation: &Operation, item: u64) -> u64 {
    match operation {
        Operation::Add(constant) => item + constant,
        Operation::Multiply(constant) => item * constant,
        Operation::Double => item + item,
        Operation::Square => item * item,
    }
}

fn count_inspections(rounds: u32, mut inventories: Vec<Inventory>, monkeys: &Vec<Monkey>, attenuation: Option<fn(u64) -> u64>) -> Vec<u64> {
    let n = inventories.len();
    let supermod: u64 = monkeys.iter().map(|m| m.divisor).product();

    let mut inspections: Vec<u64> = vec![0; n];
    for _ in 0..rounds {
        for id in 0..n {
            let monkey = &monkeys[id];
            let changes: Vec<(usize, u64)> = inventories[id].iter().map(|&item| {
                let worry = compute_worry(&monkey.operation, item) % supermod;
                let item = match attenuation {
                    Some(f) => f(worry),
                    _ => worry,
                };
                inspections[id] += 1;
                let receiver_id = if item % monkey.divisor == 0 { monkey.if_branch } else { monkey.else_branch };
                (receiver_id, item)
            }).collect();
            inventories[id].clear();
            for change in changes {
                inventories[change.0].push(change.1);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));

    inspections
}

pub fn solve() {
    let input = fs::read_to_string("resources/day11.txt").unwrap();
    let (inventories, monkeys) = parse(input);

    let inspections = count_inspections(20, inventories.clone(), &monkeys, Some(|item| item / 3));
    println!("{}", inspections[0] * inspections[1]);

    let inspections = count_inspections(10000, inventories.clone(), &monkeys, None);
    println!("{}", inspections[0] * inspections[1]);
}
