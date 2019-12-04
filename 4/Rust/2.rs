use std::collections::HashMap;

fn check_number(mut n: i32) -> bool {

    let mut map = HashMap::new();
    let mut last_digit = 10;

    while n > 0 {
        let digit = n % 10;

        if digit > last_digit {
            return false;
        }

        let count = map.entry(digit).or_insert(0);
        *count += 1;

        last_digit = digit;
        n /= 10;
    }

    for (_, value) in map {
        if value == 2 {
            return true;
        } 
    }

    false
}

fn main() {
    let mut count = 0;
    for i in 206938..679128 {
        if check_number(i) {
            count += 1;
        }
    }

    println!("{}", count);
}