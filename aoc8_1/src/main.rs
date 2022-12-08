use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = text.split('\n');

    let mut heights: Vec<Vec<u32>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut row: Vec<u32> = Vec::new();
        let mut vis: Vec<bool> = Vec::new();
        let row_chars = line.chars();
        for c in row_chars {
            row.push(match c.to_digit(10) {
                Some(d) => d,
                _ => panic!("Not a digit!"),
            });
            vis.push(true);
        }
        heights.push(row);
        visible.push(vis);
    }

    let n_rows = heights.len();
    let n_cols = heights[0].len();
    for i in 1..(n_rows - 1) {
        for j in 1..(n_cols - 1) {
            if !is_visible(&heights, i, j) {
                visible[i][j] = false;
            }
        }
    }
    let mut visibles = 0 as u32;
    for i in 0..n_rows {
        for j in 0..n_cols {
            visibles += visible[i][j] as u32;
        }
    }
    println!("{}", visibles);
}

fn is_visible(heights: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let h = heights[i][j];
    let n_rows = heights.len();
    let n_cols = heights[0].len();

    let mut bottom_visible = true;
    let mut top_visible = true;
    let mut left_visible = true;
    let mut right_visible = true;

    // Top edge
    for ii in 0..i {
        if heights[ii][j] >= h {
            top_visible = false;
            break;
        }
    }
    // Bottom edge
    for ii in (i + 1)..n_rows {
        if heights[ii][j] >= h {
            bottom_visible = false;
            break;
        }
    }
    // Left edge
    for jj in 0..j {
        if heights[i][jj] >= h {
            left_visible = false;
        }
    }
    // Right edge
    for jj in (j + 1)..n_cols {
        if heights[i][jj] >= h {
            right_visible = false;
        }
    }
    return top_visible || bottom_visible || left_visible || right_visible;
}
