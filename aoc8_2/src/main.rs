use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut heights: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut row: Vec<u32> = Vec::new();
        let row_chars = line.chars();
        for c in row_chars {
            row.push(match c.to_digit(10) {
                Some(d) => d,
                _ => panic!("Not a digit!"),
            });
        }
        heights.push(row);
    }

    let n_rows = heights.len();
    let n_cols = heights[0].len();

    let mut max_scen = 0 as u64;
    let mut biggest_tree = (0, 0);

    for i in 1..(n_rows - 1) {
        for j in 1..(n_cols - 1) {
            let s = scenic_distance(&heights, i, j);
            if s > max_scen {
                max_scen = s;
                biggest_tree = (i, j);
            }
        }
    }

    println!("{} ({}, {})", max_scen, biggest_tree.0, biggest_tree.1);
}

fn scenic_distance(heights: &Vec<Vec<u32>>, i: usize, j: usize) -> u64 {
    let h = heights[i][j];
    let n_rows = heights.len();
    let n_cols = heights[0].len();

    let mut bottom_visible = 0;
    let mut top_visible = 0;
    let mut left_visible = 0;
    let mut right_visible = 0;

    // Top edge
    for ii in (0..i).rev() {
        top_visible += 1;
        if heights[ii][j] >= h {
            break;
        }
    }
    // Bottom edge
    for ii in (i + 1)..n_rows {
        bottom_visible += 1;
        if heights[ii][j] >= h {
            break;
        }
    }
    // Left edge
    for jj in (0..j).rev() {
        left_visible += 1;
        if heights[i][jj] >= h {
            break;
        }
    }
    // Right edge
    for jj in (j + 1)..n_cols {
        right_visible += 1;
        if heights[i][jj] >= h {
            break;
        }
    }
    return top_visible * bottom_visible * left_visible * right_visible;
}
