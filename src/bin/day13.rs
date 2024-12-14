use std::fs;
use regex::Regex;
use num_traits::Float;

#[derive(Debug)]
struct Arcade {
    button_a_x: i32,
    button_a_y: i32,
    button_b_x: i32,
    button_b_y: i32,
    price_x: i32,
    price_y:i32,
}

impl Arcade {
    fn solve(&self, part: u8) -> Option<[f64;2]> {
        let a = self.button_a_x as f64;
        let b = self.button_b_x as f64;
        let c = self.button_a_y as f64;
        let d = self.button_b_y as f64;

        // Determinant 
        let det = a * d - b * c;

        if det == 0.0 {
            return None
        }
        let price_vec: [f64; 2];

        if part == 1 {
            price_vec = [self.price_x as f64, self.price_y as f64];
        } else {
            const CORRECTION: f64 = 10000000000000.0;

            price_vec = [self.price_x as f64 + CORRECTION, self.price_y as f64 + CORRECTION];
        }

        let press_a  = (d * price_vec[0] - b * price_vec[1]) / det;  
        let press_b  = (-c * price_vec[0] + a * price_vec[1]) / det;

        Some([press_a, press_b])
    }
}


fn main() {

    let mut part_1_score = 0;
    let mut part_2_score: i64 = 0;

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let puzzle = fs::read_to_string("inputs/day13.txt").expect("Couldn't reda the file!");

    for cap in re.captures_iter(&puzzle) {
        let arcade = Arcade {
            button_a_x: cap[1].parse().unwrap(),
            button_a_y: cap[2].parse().unwrap(),
            button_b_x: cap[3].parse().unwrap(),
            button_b_y: cap[4].parse().unwrap(),
            price_x: cap[5].parse().unwrap(),
            price_y: cap[6].parse().unwrap()

        };

        // PART 1
        if let Some(solution) = arcade.solve(1) {
            let a = solution[0];
            let b = solution[1];

            if is_integer(a) && is_integer(b) {
                part_1_score +=  a as i32 * 3 + b as i32;
            }
    
        }
        // PART 2
        if let Some(solution) = arcade.solve(2) {
            let a = solution[0];
            let b = solution[1];

            if is_integer(a) && is_integer(b) {
                part_2_score +=  a as i64 * 3 + b as i64;
            }
    
        }
    }
    println!("part_1_score: {part_1_score}");
    println!("part_2_score: {part_2_score}");
}

fn is_integer<T: Float>(num: T) -> bool {
    num.fract() == T::zero()
}

