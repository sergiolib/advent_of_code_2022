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
    let mut screen: Vec<Vec<char>> = Vec::new();
    initialize_screen(&mut screen, 40, 6);
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let command_cycles: u32 = get_num_cycles(line);
        let new_x: i32 = get_value_of_x_after_command(line, x);
        for _ in 0..command_cycles {
            draw_pix(&mut screen, x, cycle);
            cycle += 1;
        }
        x = new_x;
    }

    render_screen(&screen);
}

fn render_screen(screen: &Vec<Vec<char>>) {
    for row in screen {
        for pix in row {
            print!("{}", pix);
        }
        println!();
    }
}

fn draw_pix(screen: &mut Vec<Vec<char>>, x: i32, cycle: u32) {
    let width = screen[0].len() as u32;
    let i = (cycle / width) as usize;
    let j = (cycle % width) as usize;

    let c = if sprite_on(cycle, x, width) { '#' } else { '.' };
    screen[i][j] = c;
}

fn sprite_on(cycle_u: u32, x: i32, width_u: u32) -> bool {
    let cycle = cycle_u as i32;
    let width = width_u as i32;
    let mut on = (cycle % width) == x;
    if x > 0 {
        on |= (cycle % width) == x - 1;
    }
    if ((x + 1) % width) < width {
        on |= (cycle % width) == x + 1;
    }
    return on;
}

fn initialize_screen(screen: &mut Vec<Vec<char>>, width: u32, height: u32) {
    for _ in 0..height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..width {
            row.push('.');
        }
        screen.push(row);
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
