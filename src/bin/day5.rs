use rust_advent_of_code_2024::utils::{read_lines, topological_sort};
use regex::Regex;

fn main() {

    // store pairs
    let mut page_pairs: Vec<(i32, i32)> = vec![];

    let mut part1_score = 0;
    let mut part2_score = 0;

    if let Ok(lines) = read_lines("inputs/day5.txt")   {
        let re_pairs = Regex::new(r"(\d+)\|(\d+)").unwrap();
        let re_page_config = Regex::new(r"(\d+,?)*").unwrap();

        for line in lines.flatten() {

            if let Some(capture) = re_pairs.captures(&line) {
                let left = capture[1].parse::<i32>().unwrap();
                let right = capture[2].parse::<i32>().unwrap();

                page_pairs.push((left, right));
            } else if let Some(capture) = re_page_config.captures(&line) {

                // We capture the page config line as a vector of ints
                let page_config_vec: Vec<i32> = capture[0].split(",")
                    .filter(|&v| v.parse::<i32>().unwrap_or(0) != 0)
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect();
                
                // of all pairs, we're gonna use just the ones that are present in the page_config_vec
                let mut valid_page_pairs = vec![];

                for page_pair in page_pairs.clone() {
                    if page_config_vec.contains(&page_pair.0) && page_config_vec.contains(&page_pair.1) {
                        valid_page_pairs.push(page_pair);
                    }
                }

                // PART 1 
                // if topological sort = page config then append the middle value
                if let Ok(pairs_sorted) = topological_sort(&valid_page_pairs) {
                    if pairs_sorted == page_config_vec && !pairs_sorted.is_empty() {
                        let middle_num = page_config_vec[(page_config_vec.len() - 1) / 2 ];
                        part1_score += middle_num;
                    } else if !pairs_sorted.is_empty() {
                        // PART 2: there is a topological sort but isnt equal to the page config
                        // the correct page config isnt the given in the puzzle but the topological sorted one
                        // so we're gonna use that vector and append the middle value to the part 2 score
                        let middle_num = pairs_sorted[(pairs_sorted.len() - 1) / 2];
                        part2_score += middle_num;
                    }
                } 
            }
        }
    }
    println!("{part1_score}");
    println!("{part2_score}");
}