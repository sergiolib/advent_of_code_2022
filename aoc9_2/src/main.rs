use std::char;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let n_knots = 10;
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n_knots {
        knots.push((0, 0));
    }

    let mut history: Vec<HashSet<(i32, i32)>> = Vec::new();
    for _ in 0..n_knots {
        history.push(HashSet::new());
    }
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        // println!("{}", line);
        let (dir, times): (char, i32) = get_command(&line);
        for _ in 0..times {
            knots[0] = get_pos_h(dir, knots[0]);
            history[0].insert(knots[0]);
            for i in 1..n_knots {
                knots[i] = get_pos_t(knots[i - 1], knots[i]);
                history[i].insert(knots[i]);
            }
            // println!("h: ({}, {}) t: ({}, {})", h.0, h.1, t.0, t.1);
        }
    }
    println!("{}", history.last().unwrap().len());
}

fn get_pos_t(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let dist = distance(h, t);
    // println!("{}", dist);
    if dist > 1 {
        let grad = (h.0 - t.0, h.1 - t.1);
        let res = (
            if grad.0 > 0 {
                t.0 + 1
            } else if grad.0 < 0 {
                t.0 - 1
            } else {
                t.0
            },
            if grad.1 > 0 {
                t.1 + 1
            } else if grad.1 < 0 {
                t.1 - 1
            } else {
                t.1
            },
        );
        return res;
    } else {
        return t;
    }
}

fn distance(h: (i32, i32), t: (i32, i32)) -> u32 {
    let v1 = h.0 - t.0;
    let v2 = h.1 - t.1;
    return v1.abs().max(v2.abs()) as u32;
}

fn get_pos_h(dir: char, h: (i32, i32)) -> (i32, i32) {
    match dir {
        'U' => (h.0 + 1, h.1),
        'D' => (h.0 - 1, h.1),
        'R' => (h.0, h.1 + 1),
        'L' => (h.0, h.1 - 1),
        _ => h,
    }
}

fn get_command(line: &str) -> (char, i32) {
    let parts: Vec<&str> = line.split(" ").collect();
    let dir: char = parts[0].parse::<char>().unwrap();
    let times = parts[1].parse::<i32>().unwrap();
    return (dir, times);
}
