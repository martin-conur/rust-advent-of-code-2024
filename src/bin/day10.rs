use rust_advent_of_code_2024::utils::read_lines;
use std::collections::{HashSet, VecDeque};

struct Grid {
    grid: Vec<Vec<i32>>
}

impl Grid {
    fn get_width(&self) -> usize {
        self.grid[0].len() 
    }

    fn get_height(&self) -> usize {
        self.grid.len()
    }
}

fn main() {

    //  Creating the grid
    let mut grid_vec: Vec<Vec<i32>> = vec![];

    if let Ok(lines) = read_lines("inputs/day10.txt") {
        for line in lines.flatten() {
            let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(5) as i32)
            .collect();

            grid_vec.push(row);
        }

    }   

    let grid = Grid {
        grid: grid_vec
    };

    let mut part1_score = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for row in 0..grid.get_width() {
        for col in 0..grid.get_height() {
            if grid.grid[row][col] == 0 {
                part1_score += bfs(&grid, row, col, directions);
            }
        }
    }

    println!("part1 score: {part1_score}");
}

fn is_within_bounds(grid: &Grid, row: i32, col: i32) -> bool {
    return row >= 0 && row < grid.get_width() as i32 && col >= 0 && col < grid.get_height() as i32
}

fn bfs(grid: &Grid, start_row: usize, start_col: usize, directions: [(i32, i32); 4]) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut score = 0;

    queue.push_back((start_row, start_col, 0));
    visited.insert((start_row, start_col));

    while let Some((current_row, current_col, current_height)) = queue.pop_front() {

        for (neighbor_row, neighbor_col) in directions {
            let neighbor_row =  current_row as i32 + neighbor_row;
            let neighbor_col = current_col as i32 + neighbor_col;

            if is_within_bounds(&grid, neighbor_row, neighbor_col) && 
                !visited.contains(&(neighbor_row as usize, neighbor_col as usize)) &&
                grid.grid[neighbor_row as usize][neighbor_col as usize] == current_height + 1 {
                
                visited.insert((neighbor_row as usize, neighbor_col as usize));
                queue.push_front((neighbor_row as usize, neighbor_col as usize, current_height + 1));

                if grid.grid[neighbor_row as usize][neighbor_col as usize] == 9 {
                    score += 1;
                }
            }
        }
    }
    score
}