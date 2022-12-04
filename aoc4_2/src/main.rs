use std::collections::HashSet;
use std::env;
use std::fs;

fn get_range(pair: &str) -> HashSet<u32> {
    let range: Vec<&str> = pair.split("-").collect();
    let n1: u32 = range[0].parse().unwrap();
    let n2: u32 = range[1].parse().unwrap();
    let vec: Vec<u32> = (n1..(n2 + 1)).collect();
    let set: HashSet<u32> = vec.into_iter().collect();
    return set;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut result = 0;
    for line in lines {
        if line.len() == 0 {
            break;
        }
        let pairs: Vec<&str> = line.split(",").collect();
        let range1 = get_range(pairs[0]);
        let range2 = get_range(pairs[1]);
        let inter: HashSet<&u32> = range1.intersection(&range2).collect();
        let add = inter.len() > 0;
        result += add as u32;
    }
    println!("{}", result);
}
