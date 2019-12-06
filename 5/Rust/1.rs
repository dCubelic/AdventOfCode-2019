use std::io::{self, BufRead};

fn get_param(v: &mut Vec<i32>, i: usize, n: u32) -> i32 {
    let op_type = v[i] / 10_i32.pow(1+n) % 10;
    let param = if op_type == 0 { v[v[i+(n as usize)] as usize] } else { v[i+(n as usize)] };
    param
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut v: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut i = 0;
    loop {
        let opcode = v[i] % 100;
        if opcode == 99 { break }
        match opcode {
            1 => {
                let param1 = get_param(&mut v, i, 1);
                let param2 = get_param(&mut v, i, 2);
                let three = v[i+3];
                v[three as usize] = param1 + param2;
                i += 4;
            }
            2 => {
                let param1 = get_param(&mut v, i, 1);
                let param2 = get_param(&mut v, i, 2);
                let three = v[i+3];
                v[three as usize] = param1 * param2;
                i += 4;
            }
            3 => {
                let input = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
                let one = v[i+1];
                v[one as usize] = input;
                i += 2;
            }
            4 => {
                let param = v[i] / 100 % 10;
                let one = if param == 0 { v[i+1] } else { (i as i32)+1 };
                println!("{}", v[one as usize]);
                i += 2;
            }
            _ => break
        }
    }
}