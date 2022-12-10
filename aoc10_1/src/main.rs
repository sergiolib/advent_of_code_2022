use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut cycle: u32 = 0;
    let mut x = 1;
    let mut signal_strengths: HashMap<u32, i32> = HashMap::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let command_cycles: u32 = get_num_cycles(line);
        let new_x: i32 = get_value_of_x_after_command(line, x);
        for _ in 0..command_cycles {
            cycle += 1;
            get_results(cycle, x, &mut signal_strengths);
        }
        x = new_x;
    }

    let mut final_result = 0;
    for i in 0..6 {
        final_result += signal_strengths.get(&(i * 40 + 20)).unwrap();
    }
    println!("{}", final_result);
}

fn get_results(cycle: u32, x: i32, results: &mut HashMap<u32, i32>) {
    if (cycle as i32 - 20) % 40 == 0 {
        let strength = x * cycle as i32;
        results.insert(cycle, strength);
    }
}

fn get_value_of_x_after_command(line: &str, x: i32) -> i32 {
    if line.starts_with("noop") {
        x
    } else {
        let com = line.split(" ").collect::<Vec<&str>>();
        let val: i32 = com[1].parse().unwrap();
        match com[0] {
            "addx" => x + val,
            _ => panic!("Command not known"),
        }
    }
}

fn get_num_cycles(line: &str) -> u32 {
    if line.starts_with("noop") {
        1
    } else {
        let com = line.split(" ").collect::<Vec<&str>>();
        match com[0] {
            "addx" => 2,
            _ => panic!("Command not known"),
        }
    }
}
