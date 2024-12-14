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
    let mut part2_score = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    // PART 1
    for row in 0..grid.get_width() {
        for col in 0..grid.get_height() {
            if grid.grid[row][col] == 0 {
                part1_score += bfs(&grid, row, col, directions, false);
            }
        }
    }

    // PART 2
    for row in 0..grid.get_width() {
        for col in 0..grid.get_height() {
            if grid.grid[row][col] == 0 {
                part2_score += bfs(&grid, row, col, directions, true);
            }
        }
    }

    println!("part1 score: {part1_score}");
    println!("part2 score: {part2_score}");
}

fn is_within_bounds(grid: &Grid, row: i32, col: i32) -> bool {
    return row >= 0 && row < grid.get_width() as i32 && col >= 0 && col < grid.get_height() as i32
}

/// Performs Breadth-first search on the given directions
/// 
/// # Arguments
/// 
/// * `grid` - Grid struct with the grid vector.
/// * `start_row` - row index of the starting point.
/// * `start_col`- col index of the starting point.
/// * `directions` - Array with the directions to explore.
/// * `restep` - If true, you can re-visit a node from another path, otherwise, just visited once.
/// 
/// # Returns
/// 
/// The path numbers to target 
/// 
/// # Example
/// 
/// ```
/// let grid_vec: Vec<Vec<i32>> = vec![
///     vec![8, 9, 0, 1, 0, 1, 2, 3],
///     vec![7, 8, 1, 2, 1, 8, 7, 4],
///     vec![8, 7, 4, 3, 0, 9, 6, 5],
///     vec![9, 6, 5, 4, 9, 8, 7, 4],
///     vec![4, 5, 6, 7, 8, 9, 0, 3],
///     vec![3, 2, 0, 1, 9, 0, 1, 2],
///     vec![0, 1, 3, 2, 9, 8, 0, 1],
///     vec![1, 0, 4, 5, 6, 7, 3, 2],
///     ];
/// 
/// let grid = Grid {
///     grid: grid_vec
/// };
/// 
/// let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
/// let result = bfs(&grid, 0, 2, &directions, false);
/// assert_eq!(result, 5);
/// ```
fn bfs(grid: &Grid, start_row: usize, start_col: usize, directions: [(i32, i32); 4], restep: bool) -> i32 {
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
                grid.grid[neighbor_row as usize][neighbor_col as usize] == current_height + 1 &&
                restep {
                
                visited.insert((neighbor_row as usize, neighbor_col as usize));
                queue.push_front((neighbor_row as usize, neighbor_col as usize, current_height + 1));

                if grid.grid[neighbor_row as usize][neighbor_col as usize] == 9 {
                    score += 1;
                }
            } else if is_within_bounds(&grid, neighbor_row, neighbor_col) && 
            !visited.contains(&(neighbor_row as usize, neighbor_col as usize)) &&
            grid.grid[neighbor_row as usize][neighbor_col as usize] == current_height + 1 &&
            !restep {
            
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


