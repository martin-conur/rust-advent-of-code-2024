use rust_advent_of_code_2024::utils::read_lines;

fn main() {
    let mut input_vec: Vec<Vec<char>> = vec![];
    
    // a vector representing all directions of XMAS
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1), (0, 1),   (1, 1)   
    ];

    let mut x_vector: Vec<XCoords> = vec![];
    let mut a_vector: Vec<ACoords> = vec![];

    let mut part_1_score = 0;
    let mut part_2_score = 0;

    // transform input into a vector an if there is an X (part 1) or A (part 2) mark the position 
    // later we're gonna lookup this coordinates to check if XMAS is form
    if let Ok(lines) = read_lines("inputs/day4.txt") {
        for (row, line) in lines.flatten().enumerate() {
            let mut row_vector: Vec<char> = vec![];

            for (column, char_) in line.chars().enumerate() {
                // store X for part 1
                if char_ == 'X' {
                    x_vector.push(XCoords{x: column, y: row});
                }  else if char_ == 'A' && column < line.len() && row < line.len() { 
                    // if A and not in boundaries, store it for part 2
                    a_vector.push(ACoords{x: column, y: row});
                }

                row_vector.push(char_);
            }
            input_vec.push(row_vector);
        }
    }

    // part 1
    for x_coord in x_vector {
        for direction in &directions {
            if is_x_mas(&input_vec, &x_coord, direction) {
                part_1_score += 1;
            }
        }
    }

    // part 2
    for a_coord in a_vector {
        if is_xmas(&input_vec,&a_coord) {
            part_2_score += 1;
        }
    }



    println!("part1_score: {part_1_score}");
    println!("part2_score: {part_2_score}");
}
#[derive(Debug)]
struct XCoords {
    x: usize,
    y: usize
}

struct ACoords {
    x: usize,
    y: usize
}

// part 1 function
fn is_x_mas(vec: &Vec<Vec<char>>, x_coord: &XCoords, direction: &(i32, i32)) -> bool {
    let x = x_coord.x;
    let y = x_coord.y;

    // if check passed, then check if the str in the direction match 'MAS'
    let str_to_match = "MAS";

    let mut string_found = String::with_capacity(4);

    for (i, char_) in str_to_match.chars().enumerate() {
        if let Some(row) = vec.get((y as  i32 + (direction.1 * (i as i32 + 1))) as usize) {
            if let Some(vec_char) = row.get((x as  i32 + (direction.0 * (i as i32 + 1))) as usize) {
                if *vec_char == char_ {
                    string_found.push(char_);
                }
            }
        } 
    }

    if string_found.as_str() == str_to_match {
        return true
    }
    
    false
}

// part 2 function
// we're gonna check for the upper row first and check if is MS or SM
// if OK then check lower row and look for the inverse SM or MS 
fn is_xmas(vec: &Vec<Vec<char>>, a_coords: &ACoords) -> bool {

    let mut valid_diagonals = 0;

    let upper_left = (-1, -1);
    let upper_right = (1, -1);
    let lower_left = (-1, 1);
    let lower_right = (1, 1);

    let diagonal_1 = [upper_left, lower_right];
    let diagonal_2 = [upper_right, lower_left];

    for diagonal in [diagonal_1, diagonal_2] {

        let mut string_found: String = String::with_capacity(4);

        for coord_pair in diagonal {
            if let Some(row) = vec.get((a_coords.y as i32 + coord_pair.1) as usize) {
                if let Some(char_) = row.get((a_coords.x as i32 + coord_pair.0) as usize) {
                    match *char_ {
                        c if c == 'S' => {
                            string_found.push(c)
                            
                        },
                        c if c == 'M'  => {
                                string_found.push(c);
                        },
                        _ => {return false}
                    }
                }
            }
        }
        if string_found.as_str() == "MS" || string_found.as_str() == "SM" {
            valid_diagonals += 1;
        } 
    }
    matches!(valid_diagonals, 2)
}
