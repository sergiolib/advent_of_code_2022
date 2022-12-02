use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut score = 0;
    for line in lines {
        let op = match line.chars().nth(0) {
            Some(op) => op,
            _ => break,
        };
        let i = match line.chars().nth(2) {
            Some(i) => i,
            _ => break,
        };
        let shape_op = match op {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };
        let shape_i = match i {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };
        if shape_op == shape_i {
            score += 3 + shape_i
        } else if (shape_op == 1 && shape_i == 2)
            || (shape_op == 2 && shape_i == 3)
            || (shape_op == 3 && shape_i == 1)
        {
            score += 6 + shape_i
        } else {
            score += 0 + shape_i
        }
    }
    println!("Score: {}", score);
}
