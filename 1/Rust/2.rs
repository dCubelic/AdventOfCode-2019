use std::io::{self, BufRead};

fn fuel(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }

    let mut f = mass/3 - 2;
    if f < 0 {
        f = 0;
    }

    return f + fuel(f);
}

fn main() {
    let mut sum = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let number = line.unwrap().parse::<i32>().unwrap();
        sum += fuel(number);
    }

    println!("{}", sum);
}