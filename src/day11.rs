use std::fs;

enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
    Double,
}

struct MonkeyBehaviors {
    operation: Operation,
    divisor: u32,
    if_branch: usize,
    else_branch: usize,
}

type MonkeyInventory = Vec<usize>;

type Items = Vec<u32>;

fn parse(input: String) -> (Items, Vec<MonkeyInventory>, Vec<MonkeyBehaviors>) {
    let mut iter = input.lines();
    let mut inventories: Vec<MonkeyInventory> = vec![];
    let mut behaviors: Vec<MonkeyBehaviors> = vec![];
    let mut items: Vec<u32> = vec![];
    let mut item_count: usize = 0;

    loop {
        iter.next();

        let mut items_iter = iter.next().unwrap().split(": ");
        items_iter.next();
        let new_items: Vec<u32> = items_iter.next().unwrap().split(", ")
            .map(|item| u32::from_str_radix(item, 10).unwrap())
            .collect();

        let new_item_count = new_items.len();
        new_items.iter().for_each(|&item| items.push(item));

        let operation_str = iter.next().unwrap();
        let operation = if operation_str.contains('*') {
            let mut operation_iter = operation_str.split(" * ");
            operation_iter.next();
            let term_str = operation_iter.next().unwrap();
            if term_str == "old" {
                Operation::Square
            } else {
                let constant = u32::from_str_radix(term_str, 10).unwrap();
                Operation::Multiply(constant)
            }
        } else if operation_str.contains('+') {
            let mut operation_iter = operation_str.split(" + ");
            operation_iter.next();
            let term_str = operation_iter.next().unwrap();
            if term_str == "old" {
                Operation::Double
            } else {
                let constant = u32::from_str_radix(term_str, 10).unwrap();
                Operation::Add(constant)
            }
        } else {
            panic!("unhandled operation");
        };

        let test_str = iter.next().unwrap();
        let mut test_iter = test_str.split(" divisible by ");
        test_iter.next();
        let divisor = u32::from_str_radix(test_iter.next().unwrap(), 10).unwrap();

        let if_true_str = iter.next().unwrap();
        let mut if_true_iter = if_true_str.split(" throw to monkey ");
        if_true_iter.next();
        let true_monkey = usize::from_str_radix(if_true_iter.next().unwrap(), 10).unwrap();

        let if_false_str = iter.next().unwrap();
        let mut if_false_iter = if_false_str.split(" throw to monkey ");
        if_false_iter.next();
        let false_monkey = usize::from_str_radix(if_false_iter.next().unwrap(), 10).unwrap();

        inventories.push((item_count..(item_count+new_item_count)).collect::<Vec<usize>>());
        item_count += new_item_count;

        behaviors.push(MonkeyBehaviors {
            operation: operation,
            divisor: divisor,
            if_branch: true_monkey,
            else_branch: false_monkey,
        });

        match iter.next() {
            Some(_) => { },
            None => { break; },
        }
    }

    (items, inventories, behaviors)
}

fn compute_worry(operation: &Operation, item: u32) -> u32 {
    match operation {
        Operation::Add(constant) => item + constant,
        Operation::Multiply(constant) => item * constant,
        Operation::Double => item + item,
        Operation::Square => item * item,
    }
}

fn count_inspections_relaxed(items: &Items, mut inventories: Vec<MonkeyInventory>, behaviors: &Vec<MonkeyBehaviors>) -> Vec<u32> {
    const ROUNDS: usize = 20;

    let n = inventories.len();
    let mut worries: Items = items.clone();

    let mut inspections: Vec<u32> = vec![0; n];
    for _ in 0..ROUNDS {
        for id in 0..n {
            let behavior = &behaviors[id];
            let changes: Vec<(usize, usize)> = inventories[id].iter().map(|&item_idx| {
                let new_worry: u32 = compute_worry(&behavior.operation, worries[item_idx]) / 3;
                inspections[id] += 1;
                let receiver_id = if new_worry % behavior.divisor == 0 { behavior.if_branch } else { behavior.else_branch };
                worries[item_idx] = new_worry;
                (receiver_id, item_idx)
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

fn count_inspections_panic(items: &Items, mut inventories: Vec<MonkeyInventory>, behaviors: &Vec<MonkeyBehaviors>) -> Vec<u64> {
    const ROUNDS: usize = 10000;

    let n = inventories.len();
    // due to the huge amount of rounds, as well as the out of control worry levels, we try to
    // operate in the divisor "ring" of each monkey: we compute the worry level of each item
    // modulo each monkey's divisor
    let mut worries: Vec<Items> = items.iter().map(|&item| {
        behaviors.iter().map(|behavior| item % behavior.divisor).collect()
    }).collect();

    let mut inspections: Vec<u64> = vec![0; n];
    for _ in 0..ROUNDS {
        for id in 0..n {
            let behavior = &behaviors[id];
            let changes: Vec<(usize, usize)> = inventories[id].iter().map(|&item_idx| {
                let new_worry: Vec<u32> = worries[item_idx].iter().enumerate()
                    .map(|(i, &item)| compute_worry(&behavior.operation, item) % behaviors[i].divisor).collect();
                inspections[id] += 1;
                let receiver_id = if new_worry[id] == 0 { behavior.if_branch } else { behavior.else_branch };
                worries[item_idx] = new_worry;
                (receiver_id, item_idx)
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
    let (items, inventories, behaviors) = parse(input);

    let inspections = count_inspections_relaxed(&items, inventories.clone(), &behaviors);
    println!("{}", inspections[0] * inspections[1]);


    let inspections = count_inspections_panic(&items, inventories.clone(), &behaviors);
    println!("{}", inspections[0] * inspections[1]);
}
