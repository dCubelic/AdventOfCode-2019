use std::io::{self, BufRead};
use std::collections::HashSet;

fn position_set(commands: Vec<(String, i32)>) -> HashSet<(i32, i32)> {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;

    let mut set = HashSet::new();
    for (command, value) in commands {
        let old_horizontal_position = horizontal_position;
        let old_vertical_position = vertical_position;

        match command.as_ref() {
            "U" => { vertical_position += value }
            "D" => { vertical_position -= value }
            "L" => { horizontal_position -= value }
            "R" => { horizontal_position += value }
            _ => {println!("{}{}",command, value)}
        }

        let h_range;
        if old_horizontal_position < horizontal_position {
            h_range = old_horizontal_position..horizontal_position
        } else {
            h_range = horizontal_position..old_horizontal_position
        }
        for x in h_range {
            set.insert((x,old_vertical_position));
        }

        let v_range;
        if old_vertical_position < vertical_position {
            v_range = old_vertical_position..vertical_position
        } else {
            v_range = vertical_position..old_vertical_position
        }
        for y in v_range {
            set.insert((old_horizontal_position, y));
        }
    }

    set
}

fn get_commands() -> Vec<(String, i32)> {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let commands: Vec<(String, i32)> = input.split(",").map(|x| {
        let command = &x[0..1];
        let str_value = &x[1..];
        let mut command_string = String::new();
        command_string.push_str(command);
        (command_string, str_value.parse::<i32>().unwrap())
    }).collect();

    commands
}

fn main() {
    let mut first_set = position_set(get_commands());
    let second_set = position_set(get_commands());

    first_set.remove(&(0, 0));
    let intersections: HashSet<_> = first_set.intersection(&second_set).collect();

    let mut min = 10000;
    for intersection in intersections {
        if intersection.0.abs() + intersection.1.abs() < min {
            min = intersection.0.abs() + intersection.1.abs();
        }
    }
    println!("{}",min);
}