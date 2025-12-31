use std::fs;

fn main() {
    println!("Problem one:");
    prob_one();
    println!("Problem two:");
    prob_two();
}

fn prob_one() {
    let instructions = fs::read_to_string("src/input.txt").expect("Couldn't read input file");

    let mut current_num: i32 = 50;

    let mut count: u64 = 0;

    for instruction in instructions.lines() {
        let (direction, num_str) = instruction.split_at(1);
        let num: i32 = num_str.parse().expect("Couldn't parse number");
        if direction == "L" {
            current_num = (current_num - num) % 100;
        } else if direction == "R" {
            current_num = (current_num + num) % 100;
        } else {
            panic!("Couldnt read direction");
        }
        if current_num == 0 {
            count = count + 1;
        }
    }
    println!("Password is {}", count);
}

fn prob_two() {
    let instructions = fs::read_to_string("src/input.txt").expect("Couldn't read input file");

    let mut current_num: i32 = 50;

    let mut count: u64 = 0;

    for instruction in instructions.lines() {
        let (direction, num_str) = instruction.split_at(1);
        let num: i32 = num_str.parse().expect("Couldn't parse number");
        let temp_count;
        (temp_count, current_num) = rotate(direction, num, current_num);
        count = count + temp_count;
    }
    println!("Password is {}", count);
}

fn rotate(direction: &str, steps: i32, mut current_num: i32) -> (u64, i32) {
    let mut count: u64 = 0;
    match direction {
        "L" => {
            for _ in 0..steps {
                current_num = (current_num - 1) % 100;
                if current_num == 0 {
                    count = count + 1;
                }
            }
        }
        "R" => {
            for _ in 0..steps {
                current_num = (current_num + 1) % 100;
                if current_num == 0 {
                    count = count + 1;
                }
            }
        }
        _ => {
            panic!("Couldnt read direction!!!!")
        }
    }
    (count, current_num)
}
