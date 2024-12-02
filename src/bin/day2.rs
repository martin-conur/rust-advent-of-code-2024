use rust_advent_of_code_2024::utils::read_lines;

fn check_report(report: &Vec<i32>) -> (bool, usize) {
    // tracking ascenging/descending and bounds
    let mut is_valid_report = true;
    let mut last_step: Option<i32> = None;
    let mut last_level_achieved = 0;

    for ((idx, i), j) in report[..report.len() - 1]
        .iter()
        .enumerate()
        .zip(report[1..report.len()].iter()) {

            last_level_achieved = idx;

            let difference = i - j;

            // if out of bond, invalid report
            if difference > 3 || difference == 0 || difference < -3 {
                return (false, idx)
            } 

            // check if we're ascending or descending
            let current_step = match difference {
                d if d < 0 => -1,
                d if d > 0 => 1,
                _ => 0
            };

            match (current_step, last_step) {
                // if first value is ok whatever step
                (_, None) => {
                    is_valid_report = true;
                    last_step = Some(current_step);
                },
                // if we're in the same direction its ok
                (cs, Some(ls)) if cs == ls => {
                    is_valid_report = true;
                    last_step = Some(current_step);
                },
                // anything alse, bad report
                (_, _) => {
                    return (false, idx)
                }
            }
        };
        (is_valid_report, last_level_achieved)
    }

fn remove_report_level(report: &Vec<i32>, idx: usize) -> Vec<i32> {
    report
    .iter()
    .enumerate()
    .filter(|(i, _)| i != &idx)
    .map(|(_, val)| *val)
    .collect()
}

fn main() {

    // PART 1
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        // PART 1
        let part1_safe_report_counter: i32 = lines.flatten()
            .fold(0, |acc, line| {

                // parse the string report into i32 vector
                let report = line
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().expect("Expected a line of numbers!"))
                    .collect::<Vec<i32>>();
                
                let (is_valid_report, _) = check_report(&report);
                
                if is_valid_report {
                    acc + 1
                } else {
                    acc
                }
            });

        println!("part 1: {:?}", part1_safe_report_counter);
    }

    // PART 2
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        let part2_safe_report_counter: i32 = lines.flatten()
            .fold(0, |acc, line| {

                // parse the string report into i32 vector
                let report = line
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().expect("Expected a line of numbers!"))
                    .collect::<Vec<i32>>();

                let mut skipped_levels = 0;
                let mut is_valid_report = true;

                //  if check_report is false and I havent skipped any level
                //  then check without the level that causes problems
                // if otherwise, already skipped a level, report validity is false.
                if let (false, idx) = check_report(&report) {
                    skipped_levels += 1;

                    if skipped_levels > 1 {
                        is_valid_report = false;
                    } else {
                        
                        // a vector without the index with problems
                        let report_wo_idx = remove_report_level(&report, idx);
                        let report_wo_idx1 = remove_report_level(&report, idx + 1);  

                        //  if we're not on the first index, check the report removing the previous value
                        // as default we're not cheking for this
                        let mut check_report_previous_index = false;

                        if idx > 0 {
                            let report_previous_index = remove_report_level(&report, idx - 1);
                            check_report_previous_index = check_report(&report_previous_index).0;
                        } 

                        is_valid_report = check_report(&report_wo_idx).0 || check_report(&report_wo_idx1).0 || check_report_previous_index ; 
                    }
                }

                if is_valid_report {
                    acc + 1
                } else {
                    acc
                }
            });
            println!("part 2: {:?}", part2_safe_report_counter);
    }


}