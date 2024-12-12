use num::Integer;
use rust_advent_of_code_2024::utils::read_line_as_vec;

fn main() {
    let input_vector: Vec<i32> = read_line_as_vec("inputs/day11.txt");

    let mut stone_wall: Vec<i64> = input_vector.iter().map(|v| *v as i64).collect();
    for _ in  0..75 {
        stone_wall = blink(&stone_wall);
    }

    println!("{:?}", stone_wall.len());
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
    (left,right)
}

fn blink(original_vec: &Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = vec![];

    for value in original_vec {
        match value {
            0 => {
                new_stones.push(1)
            },
            num if is_length_even(&num) => {
                let (left, right) = split_even_length(&num);
                new_stones.push(left);
                new_stones.push(right);

            },
            _ => new_stones.push(value * 2024)
        };
    }

    new_stones
}
