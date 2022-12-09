use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        // println!("{}", line);
        let (dir, times): (char, i32) = get_command(&line);
        for _ in 0..times {
            // visualize(h, t, 6, 6, &history);
            h = get_pos_h(dir, h);
            t = get_pos_t(h, t);
            history.insert(t);
            // println!("h: ({}, {}) t: ({}, {})", h.0, h.1, t.0, t.1);
        }
    }
    println!("{}", history.len());
}

fn visualize(
    h: (i32, i32),
    t: (i32, i32),
    height: i32,
    weight: i32,
    history: &HashSet<(i32, i32)>,
) {
    for i in (0..height).rev() {
        for j in 0..weight {
            let c = if i == h.0 && j == h.1 {
                'H'
            } else if i == t.0 && j == t.1 {
                'T'
            } else if history.contains(&(i, j)) {
                '*'
            } else {
                '0'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
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
