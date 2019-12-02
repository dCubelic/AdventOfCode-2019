use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut splits: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    for i in (0..splits.len()).step_by(4) {
        if splits[i] == 99 {
            break
        }
        let one = splits[i+1];
        let two = splits[i+2];
        let three = splits[i+3];
        match splits[i] {
            1 => {splits[three as usize] = splits[one as usize] + splits[two as usize]}
            2 => {splits[three as usize] = splits[one as usize] * splits[two as usize]}
            _ => break
        }
    }

    println!("{}", splits[0])
}