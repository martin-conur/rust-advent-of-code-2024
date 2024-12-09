use rust_advent_of_code_2024::utils::read_lines;
use regex::Regex;

fn main() {

    let mut part_1_score: i64 = 0;
    let mut part_2_score: i64 = 0;

    let re = Regex::new(r"(\d+):\s+(.*)").unwrap();
    if let Ok(lines) = read_lines("inputs/day7.txt") {

        for line in lines.flatten() {
            if let Some(capture) = re.captures(&line) {

                let result = capture[1].parse::<i64>().unwrap();

                let equation: Vec<i64> = capture[2]
                    .split_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect();

                // if ok with normal calculations use it for part 1 and 2
                if validate_test(&equation, result) {
                    part_1_score += result as i64;
                    part_2_score += result as i64;

                } else if validate_test_2(&equation, result) {
                    // if doesnt pass first check, try concatenating and add it for
                    // part 2 score
                    part_2_score += result as i64;
                }

            }
        }
    }

    println!("part1_score: {part_1_score}");
    println!("part1_score: {part_2_score}");

}

fn validate_test(numbers: &[i64], stop_value: i64) -> bool {
    fn recursive_operations(num: &[i64], current_result: i64, combinations: &mut Vec<i64>) {
        if num.is_empty() {
            combinations.push(current_result);
            return;
        }

        let next_num = &num[0];
        let rest = &num[1..];

        // perform addition
        recursive_operations(rest, next_num + current_result, combinations);

        // perform multiplication
        recursive_operations(rest, next_num * current_result, combinations);
    }

    let mut combinations: Vec<i64> = vec![];

    if !numbers.is_empty() || combinations.contains(&stop_value) {
        recursive_operations(&numbers[1..], numbers[0], &mut combinations);
    }

    if combinations.contains(&stop_value) {
        return true
    }
    false
}


fn validate_test_2(numbers: &[i64], stop_value: i64) -> bool {
    fn recursive_operations(num: &[i64], current_result: i64, combinations: &mut Vec<i64>) {
        if num.is_empty() {
            combinations.push(current_result);
            return;
        }

        let next_num: &i64 = &num[0];
        let rest = &num[1..];

        // perform addition
        recursive_operations(rest, next_num + current_result, combinations);

        // perform multiplication
        recursive_operations(rest, next_num * current_result, combinations);

        // Perform concatenation
        if let Ok(concatenated) = format!("{}{}", current_result, next_num).parse::<i64>() {
            recursive_operations(rest, concatenated, combinations);
        }
    }

    let mut combinations: Vec<i64> = vec![];

    if !numbers.is_empty() || combinations.contains(&stop_value) {
        recursive_operations(&numbers[1..], numbers[0], &mut combinations);

    }

    if combinations.contains(&stop_value) {
        return true
    }
    false
}
