use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading {}", file_path);
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let s = 4;
    let mut msg: Vec<char> = "a".repeat(s).chars().collect();
    let mut pos = 0;
    for (in_pos, c) in text.char_indices() {
        msg[in_pos % s] = c;
        pos = in_pos;
        if in_pos > (s - 1) {
            let mut smsg = msg.clone();
            smsg.sort_unstable();
            smsg.dedup();
            // println!("smsg: {}", String::from_iter(&smsg));

            if smsg.len() == s {
                break;
            }
        }
        // println!("{}", String::from_iter(&msg));
    }
    println!("{}", pos + 1);
}
