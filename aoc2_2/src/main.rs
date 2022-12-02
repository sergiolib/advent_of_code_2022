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
        let correct_play = match op {
            'A' => 2,
            'B' => 3,
            'C' => 1,
            _ => 0,
        };
        let draw_play = match op {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };
        let wrong_play = match op {
            'A' => 3,
            'B' => 1,
            'C' => 2,
            _ => 0,
        };
        let res = match i {
            'Z' => correct_play + 6,
            'Y' => draw_play + 3,
            'X' => wrong_play + 0,
            _ => 0,
        };
        score += res;
    }
    println!("Score: {}", score);
}
