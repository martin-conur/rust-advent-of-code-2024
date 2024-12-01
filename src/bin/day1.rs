use regex::Regex;
use rust_advent_of_code_2024::utils::read_lines;
use std::collections::HashMap;

fn main() {
    // the approach will be to use regex to catch each number pair and store it into
    // two different vectors. Then sort both vectors and then calculate the distance
    // between both vectors and sum it.

    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("inputs/day1.txt") {
        for line in lines.flatten() {
            let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

            if let Some(capture) = re.captures(&line) {
                let first_list_number = capture[1].parse::<i32>().unwrap();
                let second_list_number = capture[2].parse::<i32>().unwrap();

                first_list.push(first_list_number);
                second_list.push(second_list_number);
            }
        }
    }

    // PART 1

    // sort
    first_list.sort();
    second_list.sort();

    // get pair distance and the sum
    let total_distance: i32 = first_list
        .iter()
        .zip(second_list.iter())
        .fold(0, |acc, (i, j)| acc + (i - j).abs());

    // PART 2
    // create a hashmap of second list value counts
    let mut location_frecuency: HashMap<i32, i32> = HashMap::new();

    for key in second_list {
        location_frecuency
            .entry(key)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    let similarity_score = first_list.iter().fold(0, |acc, val| {
        let frecuency = location_frecuency.get(val).unwrap_or(&0);
        acc + (val * frecuency)
    });

    println!("part1: {total_distance}");
    println!("part2: {similarity_score}");
}
