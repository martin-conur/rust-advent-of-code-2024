use std::fs;

use num::Integer;

fn main() {
    let disk_map = fs::read_to_string("sample_inputs/day9.txt").unwrap();

    let mut uncompacted_disk: Vec<String> = vec![];
    let mut id = 0;

    for (i, char) in disk_map.chars().enumerate() {
        let mut symbol = ".".into();
        if i.is_even() {
            symbol = id.to_string();
        } else {
            id += 1;
        }

        for _ in 0..char.to_digit(10).unwrap() {
            uncompacted_disk.push(symbol.clone());
        }

    }

    // PART 1:
    // two pointers solution
    let mut left: usize = 0;
    let mut right: usize = uncompacted_disk.len() - 1;

    while left < right {
        if  uncompacted_disk[right] == "." {
            right -= 1;
        }
        else if  uncompacted_disk[left] == "." {
            uncompacted_disk.swap(left, right);
            right -= 1;

        } else if uncompacted_disk[left] != "." {
            left += 1;
        }
    }

    let score_part_1: i64 = uncompacted_disk
        .iter()
        .filter(|v| *v != ".")
        .enumerate()
        .fold(0, |acc, (i, val)| acc + i as i64 * val.parse::<i64>().unwrap());

    println!("{:?}", score_part_1);
    // println!("{:?}", uncompacted_disk);

    // PART 2:
    // store free space indices
    let mut disk_map_free_space: Vec<usize> = disk_map
        .chars()
        .enumerate()
        .map(|(i, _)| {
            if i.is_even() {return 0}
            1
        })
        .collect();

    // store data id 
    let mut disk_map_data: Vec<usize> = disk_map
    .chars()
    .enumerate()
    .scan(0,|state, (i, _)| {
        if i == 0 {
            Some(*state)
        } else if i.is_even() {
            *state += 1;
            Some(*state)
        }
        else {Some(0)}
    })
    .collect();

    println!("{:?}", disk_map_data);
    //println!("{:?}", disk_map_free_space);

}