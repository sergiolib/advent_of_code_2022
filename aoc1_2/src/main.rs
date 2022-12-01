use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');
    let mut max_calories_1 = 0;
    let mut max_calories_2 = 0;
    let mut max_calories_3 = 0;
    let mut _calories = 0;
    for line in lines {
        if line.is_empty() {
            if _calories > max_calories_1 {
                max_calories_3 = max_calories_2;
                max_calories_2 = max_calories_1;
                max_calories_1 = _calories;
            } else if _calories > max_calories_2 {
                max_calories_3 = max_calories_2;
                max_calories_2 = _calories;
            } else if _calories > max_calories_3 {
                max_calories_3 = _calories;
            }
            _calories = 0;
        } else {
            let val = line.parse::<u32>().unwrap();
            _calories += val;
        }
    }
    println!("Max calories 1: {}", max_calories_1);
    println!("Max calories 2: {}", max_calories_2);
    println!("Max calories 3: {}", max_calories_3);
    println!(
        "Summed calories: {}",
        max_calories_1 + max_calories_2 + max_calories_3
    )
}
