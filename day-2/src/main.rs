use std::fs;

fn main() {
    println!("Problem one:");
    prob_one();
    println!("Problem two:");
    prob_two();
}

fn prob_one() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read input");
    let ranges: Vec<&str> = input.trim().split(",").collect();
    let mut faulty_ids_addition: u64 = 0;
    for range in ranges {
        let (start_str, finish_str) = range.split_once("-").expect("Could not split id range");
        let start: u64 = start_str
            .parse()
            .expect("Could not convert start to number");
        let finish: u64 = finish_str
            .parse()
            .expect("Could not convert finish to number");
        for current in start..finish {
            let current_str = format!("{}", current);
            let mid = current_str.chars().count() / 2;
            let (first, second) = current_str.split_at(
                current_str
                    .char_indices()
                    .nth(mid)
                    .map(|(i, _)| i)
                    .expect("Could not split current num in half"),
            );
            if first == second {
                faulty_ids_addition = faulty_ids_addition + current;
            }
        }
    }
    println!("Sum of faulty ids: {}", faulty_ids_addition);
}

fn prob_two() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read input");
    let ranges: Vec<&str> = input.trim().split(",").collect();
    let mut faulty_ids_addition: u64 = 0;
    for range in ranges {
        let (start_str, finish_str) = range.split_once("-").expect("Could not split id range");
        let start: u64 = start_str
            .parse()
            .expect("Could not convert start to number");
        let finish: u64 = finish_str
            .parse()
            .expect("Could not convert finish to number");
        for current in start..finish {
            let current_str = format!("{}", current);
            if !is_valid(current_str.as_str()) {
                faulty_ids_addition = faulty_ids_addition + current;
            }
        }
    }
    println!("Sum of faulty ids: {}", faulty_ids_addition);
}

fn is_valid(number: &str) -> bool {
    let range_len = number.len() / 2;
    for index in 1..range_len + 1 {
        let indices: Vec<&str> = number
            .as_bytes()
            .chunks(index)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();
        let is_invalid = indices.iter().all(|&x| x == indices[0]);
        if is_invalid {
            return false;
        }
    }
    return true;
}
