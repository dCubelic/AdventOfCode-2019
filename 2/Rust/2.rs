use std::io::{self, BufRead};

fn calculate(mut v: Vec<i32>) -> i32 {
    for i in (0..v.len()).step_by(4) {
        if v[i] == 99 { break }
        let three = v[i+3];
        match v[i] {
            1 => { v[three as usize] = v[v[i+1] as usize] + v[v[i+2] as usize]}
            2 => { v[three as usize] = v[v[i+1] as usize] * v[v[i+2] as usize]}
            _ => break
        }
    }

    v[0]
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let splits: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut v = splits.clone();
            v[1] = noun;
            v[2] = verb;
            if calculate(v) == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}