use std::fs;

fn main() {
    println!("Problem one:");
    prob_one();
    println!("Problem two:");
    prob_two();
}

fn prob_one() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let mut final_answer = 0;
    for jolt in input.lines() {
        let digits: Vec<u64> = jolt
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let mut highest = 0;
        for (index, digit) in digits.iter().enumerate().take(digits.len() - 1) {
            let highest_second = *digits[index + 1..]
                .iter()
                .max()
                .expect("Could not find highest");
            let temp_highest = digit * 10 + highest_second;
            if temp_highest > highest {
                highest = temp_highest;
            }
        }
        final_answer = final_answer + highest;
    }
    println!("Code is: {}", final_answer);
}

fn prob_two() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let mut final_answer: u128 = 0;
    for jolt in input.lines() {
        let digits: Vec<u64> = jolt
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let mut final_answer_vec: Vec<u64> = Vec::new();
        let mut start: usize = 0;

        while final_answer_vec.len() < 12 {
            let picked = final_answer_vec.len();
            let end_inclusive = digits.len() - (12 - picked);

            let mut best_digit = 0u64;
            let mut best_idx = start;

            for i in start..=end_inclusive {
                if digits[i] > best_digit {
                    best_digit = digits[i];
                    best_idx = i;
                    if best_digit == 9 {
                        break;
                    }
                }
            }

            final_answer_vec.push(best_digit);
            start = best_idx + 1;
        }
        let mut res: u128 = 0;

        for i in 0..final_answer_vec.len() {
            let temp: u128 =
                (final_answer_vec[i] * 10u64.pow((final_answer_vec.len() - 1 - i) as u32)) as u128;
            res = res + temp;
        }
        final_answer = final_answer + res;
    }
    println!("Code is: {}", final_answer);
}
