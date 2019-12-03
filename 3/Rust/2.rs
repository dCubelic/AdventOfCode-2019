use std::io::{self, BufRead};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    steps: i32
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn position_set(commands: Vec<(String, i32)>) -> HashSet<(Position)> {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;

    let mut set = HashSet::new();
    let mut step = 0;
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

        if old_horizontal_position < horizontal_position {
            for x in old_horizontal_position..horizontal_position {
                step += 1;
                set.insert(Position { x: x+1, y: old_vertical_position, steps: step });
            }
        } else {
            for x in (horizontal_position-1..old_horizontal_position-1).rev() {
                step += 1;
                set.insert(Position { x: x+1, y: old_vertical_position, steps: step });
            }
        }

        if old_vertical_position < vertical_position {
            for y in old_vertical_position..vertical_position {
                step += 1;
                set.insert(Position { x: old_horizontal_position, y: y+1, steps: step });
            }
        } else {
            for y in (vertical_position-1..old_vertical_position-1).rev() {
                step += 1;
                set.insert(Position { x: old_horizontal_position, y: y+1, steps: step });
            }
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

    first_set.remove(&Position {x: 0, y: 0, steps: 0});
    let intersections: HashSet<_> = first_set.intersection(&second_set).collect();

    let mut min = 1000000;
    for intersection in intersections {
        let position = first_set.get(intersection).unwrap();
        let position2 = second_set.get(intersection).unwrap();
        if position.steps + position2.steps < min {
            min = position.steps + position2.steps
        }
    }
    
    println!("{}",min);
}