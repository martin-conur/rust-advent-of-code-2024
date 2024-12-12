use std::collections::HashMap;
use num::Integer;
use rayon::prelude::*;
use rust_advent_of_code_2024::utils::read_line_as_vec;

fn main() {
    let input_vector: Vec<i32> = read_line_as_vec("inputs/day11.txt");

    let result: i64 = input_vector
        .par_iter()
        .map(|&v| {
            let mut local_cache = HashMap::new(); // Each thread gets its own cache
            blink(v as i64, 0, 75, &mut local_cache)
        })
        .sum();

    println!("{result}");
}

fn is_length_even(number: &i64) -> bool {
    (((*number as f64).log10() as usize) + 1).is_even()
}

fn split_even_length(number: &i64) -> (i64, i64) {
    let total_digits = ((*number as f64).log10() as usize) + 1;
    let right_digits = total_digits / 2;
    let divisor = 10_i32.pow(right_digits as u32) as i64;

    let left = number / divisor;
    let right = number % divisor;
    (left, right)
}

fn blink(
    stone_number: i64,
    blinks: u32,
    stop_at: u32,
    cache: &mut HashMap<(i64, u32), i64>,
) -> i64 {
    if blinks >= stop_at {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(stone_number, blinks)) {
        return cached_result;
    }

    let result = match stone_number {
        0 => blink(1, blinks + 1, stop_at, cache),
        num if is_length_even(&num) => {
            let (left, right) = split_even_length(&num);
            blink(left, blinks + 1, stop_at, cache)
                + blink(right, blinks + 1, stop_at, cache)
        }
        _ => blink(stone_number * 2024, blinks + 1, stop_at, cache),
    };

    cache.insert((stone_number, blinks), result);

    result
}
