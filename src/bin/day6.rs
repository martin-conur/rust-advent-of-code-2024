use std::collections::HashMap;
use rust_advent_of_code_2024::utils::read_lines;

#[derive(Debug, Clone, PartialEq)]
enum LookingDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Guard {
    looking_direction: LookingDirection,
    position: (usize, usize),
    inside_map: bool,
    places_visited: usize,
    loop_mode: bool
}

// Map grid with assiciated metadata
#[derive(Debug, Clone)]
struct Map {
    map: HashMap<(usize, usize), char>,
    width: usize,
    height: usize
}

#[derive(PartialEq)]
struct MapDirections {
    map: HashMap<(usize, usize), Vec<LookingDirection>>
}

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
            height: 0,
            width: 0
        }
    }

    fn update_height(&mut self, new_height: usize) {
        self.height = new_height
    }

    fn update_width(&mut self, new_width: usize) {
        self.width = new_width
    }

    fn insert_map(&mut self, position: (usize, usize), value: char) {
        self.map.insert(position, value);
    }
}

impl MapDirections {
    fn new() -> Self {
        MapDirections {
            map: HashMap::new(),
        }
    }

    fn insert_visited_direction(&mut self, position: (usize, usize), direction: LookingDirection) {
        if let Some(vec_direction) = self.map.get(&position) {
        let mut new_vec = vec_direction.clone();
        new_vec.push(direction);

            self.map.insert(position, new_vec)
        } else {
            self.map.insert(position, vec![direction])
        };
    }
}


impl Guard {
    fn new(looking_direction: LookingDirection, position: (usize, usize)) -> Self {
        Guard {
            looking_direction,
            position,
            inside_map: true,
            places_visited: 1, //starting position
            loop_mode: false
        }
    }

    fn turn_right(&mut self) {
        match self.looking_direction {
            LookingDirection::Down => self.looking_direction = LookingDirection::Left,
            LookingDirection::Left => self.looking_direction = LookingDirection::Up,
            LookingDirection::Up => self.looking_direction = LookingDirection::Right,
            LookingDirection::Right => self.looking_direction = LookingDirection::Down
        }
    }

    fn set_outside_map(&mut self) {
        self.inside_map = false
    }

    fn advance(&mut self, map: &mut Map) {

        match map.map.get(&self.position).unwrap() {
            '.' => {
                // change map new place to already visited
                map.insert_map(self.position, 'X');

                // update visited places
                self.places_visited += 1;
            },
            _ => {} 
        }

        let step: (i32, i32) = match self.looking_direction {
            LookingDirection::Down => (0, 1),
            LookingDirection::Left => (-1, 0),
            LookingDirection::Right => (1, 0),
            LookingDirection::Up => (0, -1)
        };



        // check if we are outside the map in the next step
        // if indeed we're outside the map, then put the guard outside the map
        match (step.0 + self.position.0 as i32, step.1 + self.position.1 as i32) {
            (x, y) if x < 0 || y < 0 => {
                self.set_outside_map();
                return 
            },
            (x, y) if x >= map.width as i32|| y >= map.height as i32 => {
                self.set_outside_map();                
                return 
            },
            front_position => {
                let front_position = (front_position.0 as usize, front_position.1 as usize);

                // check what's ahead
                let in_front = map.map.get(&front_position).unwrap();

                match *in_front {
                    '.' | 'X' | '|' | '-' | '+' => {
                        // update position 1 step forward
                        self.position = ((self.position.0 as i32 + step.0) as usize, (self.position.1 as i32 + step.1) as usize);
                    },
                    '#' => self.turn_right(),
                    _ => {}
                }
            }
        } 

    }

    // for part 2
    fn advance_with_obstruction(&mut self, map: &mut Map, map_directions: &mut MapDirections) {

        if let Some(dir) = map_directions.map.get(&self.position) {
            if dir.contains(&self.looking_direction) {
                self.loop_mode = true;
            }
        }

        let char_direction = match &self.looking_direction {
            LookingDirection::Down | LookingDirection::Up => '|',
            LookingDirection::Left | LookingDirection::Right => '-'
        };

        match (map.map.get(&self.position).unwrap(), char_direction) {

            ('.' | 'X',direction) => {
                // change map new place to current direction
                map.insert_map(self.position, direction);
                map_directions.insert_visited_direction(self.position, self.looking_direction.clone());
            },
            // if already passed here in the same direction
            (past_direction, direction) if past_direction == &direction => {
                map_directions.insert_visited_direction(self.position, self.looking_direction.clone());
            },
            // if this time i pass in perpendicular direction
            (past_direction, direction) if past_direction != &direction => {
                map.insert_map(self.position, '+');
            },
            // NOTE: this never happens, maybe directions should be enum to avoid this bad practice
            _ => {} 
        }

        let step: (i32, i32) = match self.looking_direction {
            LookingDirection::Down => (0, 1),
            LookingDirection::Left => (-1, 0),
            LookingDirection::Right => (1, 0),
            LookingDirection::Up => (0, -1)
        };



        // check if we are outside the map in the next step
        // if indeed we're outside the map, then put the guard outside the map
        match (step.0 + self.position.0 as i32, step.1 + self.position.1 as i32) {
            (x, y) if x < 0 || y < 0 => {
                self.set_outside_map();
                return 
            },
            (x, y) if x >= map.width as i32|| y >= map.height as i32 => {
                self.set_outside_map();                
                return 
            },
            front_position => {

                let front_position = (front_position.0 as usize, front_position.1 as usize);

                // check what's ahead
                let in_front = map.map.get(&front_position).unwrap();

                match *in_front {
                    '.' | 'X' | '|' | '-' | '+' => {
                        // update position 1 step forward
                        self.position = ((self.position.0 as i32 + step.0) as usize, (self.position.1 as i32 + step.1) as usize);
                    },
                    '#' => self.turn_right(),
                    _ => {}
                }
            }
        } 

    }
}


fn main() {

    let mut map = Map::new();

    let mut guard: Guard = Guard::new(LookingDirection::Up, (0, 0)); 

    if let Ok(lines) = read_lines("inputs/day6.txt") {
        for (y, line) in lines.flatten().enumerate() {
            // in the first line we can get the map width
            if y == 0 {
                map.update_width(line.len());
            }

            for (x, char_) in line.chars().enumerate() {

                // until the last line, we are increasing map height by one
                map.update_height(x + 1);
                
                // if we found the guard in the input puzzle, assign it to the struct variable
                // and put a X in that position (a visited place)

                match char_ {
                    'v' => {
                        guard = Guard::new(LookingDirection::Down, (x, y));
                        map.insert_map((x, y), 'X')
                    },
                    '>' => {
                        guard = Guard::new(LookingDirection::Right, (x, y));
                        map.insert_map((x, y), 'X')
                    },
                    '<' => {
                        guard = Guard::new(LookingDirection::Left, (x, y));
                        map.insert_map((x, y), 'X')
                    },
                    '^' => {
                        guard = Guard::new(LookingDirection::Up, (x, y));
                        map.insert_map((x, y), 'X')
                    },
                    _ => map.insert_map((x, y), char_)
                }

            }
        }
    }


    // PART 1
    let mut guard_part1 = guard.clone();
    let mut map_part1 = map.clone();

    while guard_part1.inside_map {
        guard_part1.advance(&mut map_part1);
    }

    println!("part 1 score: {}", guard_part1.places_visited);


    // PART 2: I will iterate over all posible places to put an obstruction (.)
    // store in the hashmap - or | or + to note the direction of an already visted place
    // if I encounter an already visited place with the same symbol or a + means that i'm on a loop
    // so we add 1 to the counter and go to the next possible place.
    // Otherwise if i get out the map, go to the next possible place.

    let mut possible_obstructions = 0;

    for (k, v) in map.map.iter() {
        if v != &'#'&& v!= &'X' {
    
            // create a new map with a obstruction in this position
            let mut map_part2 = map.clone();
            map_part2.insert_map(k.clone(), '#');

            let mut map_directions = MapDirections::new();

            // create a guard in the starting position
            let mut guard_part2 = guard.clone();

            while guard_part2.inside_map {

                guard_part2.advance_with_obstruction(&mut map_part2, &mut map_directions);

                if guard_part2.loop_mode {
                    possible_obstructions += 1;
                    println!("{possible_obstructions}");
                    break
                }
            }

        }
    }
    println!("score part 2: {possible_obstructions}");


}