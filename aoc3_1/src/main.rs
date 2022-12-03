use std::collections::HashSet;
use std::env;
use std::fs;

const VAL_A: u32 = 'a' as u32;
const VAL_BIG_A: u32 = 'A' as u32;
const VAL_Z: u32 = 'z' as u32;
const VAL_BIG_Z: u32 = 'Z' as u32;

fn score(c: char) -> Option<u32> {
    let char_val = c as u32;
    if (VAL_A <= char_val) && (char_val <= VAL_Z) {
        return Some(char_val - VAL_A + 1);
    } else if (VAL_BIG_A <= char_val) && (char_val <= VAL_BIG_Z) {
        return Some(char_val - VAL_BIG_A + 27);
    } else {
        println!("Duh!");
        panic!("Incorrect char {}", c);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut result = 0;
    for line in lines {
        let half_length: usize = line.chars().count() / 2;
        let mut set: HashSet<char> = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if i < half_length {
                set.insert(c);
            } else if set.contains(&c) {
                match score(c) {
                    Some(inner) => {
                        result += inner;
                        break;
                    }
                    _ => continue,
                };
            }
        }
    }
    println!("{}", result);
}
