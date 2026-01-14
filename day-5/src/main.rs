use std::fs;

fn main() {
    println!("Part 1:");
    prob_one();
    println!("Part 2:");
    prob_two();
}

#[derive(Clone)]
struct Range {
    start: u64,
    stop: u64,
}

fn prob_one() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read string");
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let mut ingredients: Vec<u64> = ingredients_str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    ingredients.sort();

    let ranges: Vec<Range> = ranges_str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (a, b) = l.trim().split_once("-").expect("Couldn't split range");
            Range {
                start: a.parse::<u64>().expect("Bad start"),
                stop: b.parse::<u64>().expect("Bad stop"),
            }
        })
        .collect();

    let mut result = 0;

    for ingredient in ingredients {
        for range in ranges.clone().into_iter() {
            if ingredient > range.start && ingredient < range.stop {
                result = result + 1;
                break;
            }
        }
    }
    println!("There is {} fresh ingredients", result);
}

fn prob_two() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read string");
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let mut ingredients: Vec<u64> = ingredients_str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    ingredients.sort();

    let ranges: Vec<Range> = ranges_str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (a, b) = l.trim().split_once("-").expect("Couldn't split range");
            Range {
                start: a.parse::<u64>().expect("Bad start"),
                stop: b.parse::<u64>().expect("Bad stop"),
            }
        })
        .collect();

    //let mut lowest = ranges[0].start;
    //let mut highest = ranges[0].stop;
    //for i in 1..ranges.len() {
    //    if ranges[i].start < lowest {
    //        lowest = ranges[i].start;
    //    }
    //    if ranges[i].stop > highest {
    //        highest = ranges[i].stop;
    //    }
    //}

    // Merge + sum union length
    let mut total: u64 = 0;
    let mut cur_start = ranges[0].start;
    let mut cur_stop = ranges[0].stop;

    for r in ranges.into_iter().skip(1) {
        if r.start <= cur_stop + 1 {
            // overlaps or touches (inclusive IDs)
            cur_stop = cur_stop.max(r.stop);
        } else if r.stop >= cur_start {
            cur_start = cur_start.min(r.start);
        } else {
            // close current interval
            total += cur_stop - cur_start + 1;
            cur_start = r.start;
            cur_stop = r.stop;
        }
    }

    // last interval
    total += cur_stop - cur_start;

    println!("There is {} fresh ingredients", total);
}
