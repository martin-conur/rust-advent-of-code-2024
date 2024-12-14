use std::collections::{HashSet, VecDeque};

use rust_advent_of_code_2024::utils::read_lines_as_str_grid;

/**
 * First test approach:
 * For any letter, track 4 directions neighbors, and apply this rule:
 *  if no neighbors same letter: 4 perimeter points
 *  if 1 neighbor same letter: 3 perimeter points
 *  if 2 neighbors same letter: 2 perimeter points
 *  if 3 neighbors same letter: 1 perimeter points
 *  if 4 neighbors same letter: 0 perimeter points
 */

 struct Grid {
    grid: Vec<Vec<char>>
 }

 impl Grid {
    fn get_height(&self) -> usize {
        self.grid.len()
    }

    fn get_width(&self) -> usize {
        self.grid[0].len()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct GardenPlot {
    garden_type: char,
    neighbors: i32,
    position: (usize, usize)
}

impl GardenPlot {
    fn add_neighbor(&mut self) {
        self.neighbors += 1
    }
}

 fn main() {
    let vec_grid = read_lines_as_str_grid("inputs/day12.txt");
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let grid = Grid {
        grid: vec_grid
    };


    let mut visited: HashSet<(usize, usize)>  = HashSet::new();

    let mut queue = VecDeque::new();

    let mut map: Vec<Vec<GardenPlot>> = Vec::new();


    for row in 0..grid.get_height() {
        for col in 0..grid.get_width() {

            if !visited.contains(&(col, row)) {
                queue.push_front((col, row));
                // visited.insert((col, row));

                let mut plot: Vec<GardenPlot> = vec![];

                while let Some((col_q, row_q)) = queue.pop_front() {
                    if visited.contains(&(col_q, row_q)) {
                        continue;
                    }
                    visited.insert((col_q, row_q));


                    let mut current_plot = GardenPlot {
                        garden_type: grid.grid[row_q][col_q],
                        neighbors:0,
                        position: (col_q, row_q)
                    };
                    
                    for (col_step, row_step) in directions {

                        let neighbor_col = col_step + col_q as i32;
                        let neighbor_row = row_step + row_q as i32;

                        if is_within_bounds(&grid, neighbor_row, neighbor_col) {
                            if current_plot.garden_type == grid.grid[neighbor_row as usize][neighbor_col as usize] {
                                current_plot.add_neighbor();

                                if !visited.contains(&(neighbor_col as usize, neighbor_row as usize)) {
                                    queue.push_front((neighbor_col as usize, neighbor_row as usize));
                                }
                            } 
                        }
                    }
                    plot.push(current_plot);
                }
                map.push(plot);

            }
        }
    }

    let part_1_score: i32 = map
        .iter()
        .map(|v| {
            v.len() as i32 * v.iter().map(|g| 4 - g.neighbors).sum::<i32>()
        })
        .sum();

    println!("part1_score: {:?}", part_1_score);


}

fn is_within_bounds(grid: &Grid, row: i32, col: i32) -> bool {
    return row >= 0 && row < grid.get_height() as i32 && col >= 0 && col < grid.get_width() as i32
}