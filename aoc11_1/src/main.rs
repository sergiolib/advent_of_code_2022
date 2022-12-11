use std::collections::VecDeque;
use std::env;
use std::fs;

struct Monkey {
    monkey_n: u32,
    items: VecDeque<Item>,
    operation_sign: char,
    operation_number: u32,
    test_divisible_by: u32,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    items_inspected: u64,
}

struct Item {
    worry_level: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split("\n\n");

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_s in lines {
        // println!("{}", monkey_s);
        let monkey: Monkey = parse_monkey(&monkey_s);
        monkeys.push(monkey);
    }
    let n_rounds = 20;
    for _round in 0..n_rounds {
        for monkey_n in 0..monkeys.len() {
            inspect_items(monkey_n, &mut monkeys);
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));

    for ele in &monkeys {
        println!("{} {}", ele.monkey_n, ele.items_inspected);
    }

    println!(
        "Result: {}",
        monkeys[0].items_inspected * monkeys[1].items_inspected
    )
}

fn inspect_items(monkey_n: usize, monkeys: &mut Vec<Monkey>) {
    while !monkeys[monkey_n].items.is_empty() {
        let mut item = monkeys[monkey_n].items.pop_front().unwrap();
        let new_worry_level = match monkeys[monkey_n].operation_sign {
            '+' => item.worry_level + monkeys[monkey_n].operation_number,
            '-' => item.worry_level - monkeys[monkey_n].operation_number,
            '*' => item.worry_level * monkeys[monkey_n].operation_number,
            '/' => item.worry_level / monkeys[monkey_n].operation_number,
            's' => item.worry_level * item.worry_level,
            _ => panic!(
                "Operation sign {} not implemented",
                monkeys[monkey_n].operation_sign
            ),
        };
        let after_bored = new_worry_level / 3;
        item.worry_level = after_bored;
        let test_result = after_bored % monkeys[monkey_n].test_divisible_by == 0;
        let monkey_dest: usize = if test_result {
            {
                monkeys[monkey_n].if_true_throw_to
            }
        } else {
            {
                monkeys[monkey_n].if_false_throw_to
            }
        };
        monkeys[monkey_dest].items.push_back(item);
        monkeys[monkey_n].items_inspected += 1;
    }
}

fn parse_monkey(action_s: &str) -> Monkey {
    let lines: Vec<&str> = action_s.split('\n').collect();

    let parsed_monkey_n: u32 = lines[0]
        .split(':')
        .nth(0)
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let parsed_starting_items: VecDeque<Item> = lines[1][18..]
        .split(", ")
        .map(|x| Item {
            worry_level: x.parse().unwrap(),
        })
        .collect();

    let line2_words: Vec<&str> = lines[2].split(' ').collect();
    let last_word: &str = line2_words.last().unwrap();
    let second_last_word: &str = line2_words.iter().nth_back(1).unwrap();
    let (parsed_operation_sign, parsed_operation_number): (char, u32) = match last_word {
        "old" => ('s', 0),
        _ => (
            second_last_word.parse::<char>().unwrap(),
            last_word.parse::<u32>().unwrap(),
        ),
    };

    let parsed_test_divisible_by: u32 = lines[3].split(' ').last().unwrap().parse().unwrap();

    let parsed_if_true_throw_to: usize = lines[4].split(' ').last().unwrap().parse().unwrap();
    let parsed_if_false_throw_to: usize = lines[5].split(' ').last().unwrap().parse().unwrap();

    let monkey = Monkey {
        monkey_n: parsed_monkey_n,
        items: parsed_starting_items,
        operation_sign: parsed_operation_sign,
        operation_number: parsed_operation_number,
        test_divisible_by: parsed_test_divisible_by,
        if_true_throw_to: parsed_if_true_throw_to,
        if_false_throw_to: parsed_if_false_throw_to,
        items_inspected: 0,
    };
    return monkey;
}
