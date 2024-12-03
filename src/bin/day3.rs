use rust_advent_of_code_2024::utils::read_lines;
use regex::Regex;


fn main() {
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\)|don't\(\))").unwrap();
        let re_inner = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
        
        let mut part_1_score = 0;
        let mut part_2_score = 0;
        let mut multiplications_enabled =true;

        
        for line in lines.flatten() {
            for capture in re.find_iter(&line) {
                
                match capture.as_str() {
                    "do()" => multiplications_enabled = true,
                    "don't()" => multiplications_enabled = false,
                    _ => {
                        if let Some(inner_capture) = re_inner.captures(capture.as_str()) {
                            let first_value = inner_capture[1].parse::<i32>().unwrap();
                            let second_value = inner_capture[2].parse::<i32>().unwrap();

                            part_1_score += first_value * second_value;

                            if multiplications_enabled {
                                part_2_score += first_value * second_value;
                            }
                        }
                    }
                }
            }
        }
        println!("part 1: {part_1_score}");
        println!("part 2: {part_2_score}");
    }
}