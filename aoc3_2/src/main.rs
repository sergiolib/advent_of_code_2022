use std::collections::HashMap;
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

    let mut result: HashMap<u32, u32> = HashMap::new();
    let mut group_set: HashSet<char> = HashSet::new();
    let mut previous_group: i32 = -1;
    for (li, line) in lines.enumerate() {
        let cur_set = HashSet::from_iter(line.chars());
        let group: i32 = (li / 3) as i32;
        if group > previous_group {
            for res in group_set.drain() {
                let v = match score(res) {
                    Some(inner) => inner,
                    _ => 0,
                };
                result.insert(previous_group as u32, v);
                break;
            }
            group_set = cur_set.clone();
        }
        group_set.retain(|f| cur_set.contains(f));
        previous_group = group;
    }
    let mut result_sum = 0;
    for v in result.values() {
        result_sum += v;
    }
    println!("{}", result_sum);
}
