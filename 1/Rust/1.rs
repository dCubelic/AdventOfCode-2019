use std::io::{self, BufRead};

fn main() {
    let mut sum = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let number = line.unwrap().parse::<i32>().unwrap();
        sum += number/3 - 2;
    }

    println!("{}", sum);
}