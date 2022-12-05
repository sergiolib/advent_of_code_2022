use std::env;
use std::fs;

fn init_add_to_stack(stack: &mut Vec<Vec<char>>, line: &str) {
    let list: Vec<char> = line.chars().collect();
    let mut i = 1;
    for j in 0..9 {
        if list[i] != ' ' {
            stack[j].insert(0, list[i]);
        }
        i += 4;
    }
}

fn move_n_from_to(stack: &mut Vec<Vec<char>>, n: u32, from: usize, to: usize) {
    for _i in 0..n {
        let elem = stack[from].pop().unwrap();
        stack[to].push(elem);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');
    let mut board = Vec::<Vec<char>>::from([
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
    ]);

    for line in lines {
        if line.starts_with("move") {
            let parts: Vec<&str> = line.split(' ').collect();
            let n: u32 = parts[1].parse().unwrap();
            let from: usize = parts[3].parse().unwrap();
            let to: usize = parts[5].parse().unwrap();
            move_n_from_to(&mut board, n, from - 1, to - 1);
        } else if line.contains("[") {
            init_add_to_stack(&mut board, line);
        } else if line == " 1   2   3   4   5   6   7   8   9 " {
            print_board(&board);
        }
    }
    println!();
    print_board(&board);
}

fn print_board(stack: &Vec<Vec<char>>) {
    for s in 0..9 {
        let inner = &stack[s];
        for i in 0..inner.len() {
            print!("{} ", stack[s][i]);
        }
        println!()
    }
}
