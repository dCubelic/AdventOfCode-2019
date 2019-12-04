fn check_number(mut n: i32) -> bool {

    let mut repeating_digit = false;
    let mut last_digit = n % 10;
    n /= 10;

    while n > 0 {
        let digit = n % 10;
        if digit == last_digit {
            repeating_digit = true;
        }
        if digit > last_digit {
            return false;
        }
        last_digit = digit;
        n /= 10;
    }

    repeating_digit
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