#![allow(dead_code)]

use std::fs::File;
use std::result::Result;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let inventory = read_file_as_vec("src/input/day01.txt".to_string());
    let res = get_most_calories(inventory);
    return res;
}

fn read_file_as_vec(path: String) -> Vec<Vec<i64>> {
    let value: Vec<String> = helpers::read(File::open(path).unwrap()).unwrap();
    let inventory_text: Vec<Vec<String>> = value
        .split(|calories| *calories == "")
        .map(|e| e.into_iter().map(|s| s.clone()).collect::<Vec<String>>())
        .collect();
    let inventory = inventory_text
        .into_iter()
        .map(|elf_inventory| {
            elf_inventory
                .into_iter()
                .map(|ration| ration.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    return inventory
}

fn get_calories(rations: &Vec<i64>) -> i64 {
    rations.into_iter().fold(0, |x, y| x + y)
}

fn get_most_calories(inventory: Vec<Vec<i64>>) -> i64 {
    inventory.iter().map(|elf| get_calories(elf)).max().unwrap()
}

// ____________________
// Part 2
// ____________________


fn sort_by_most_calories(inventory: Vec<Vec<i64>>) -> i64 {
    let mut total_calories = inventory.iter().map(|elf| get_calories(elf)).collect::<Vec<i64>>();
    total_calories.sort();
    total_calories.reverse();
    let result = total_calories.into_iter().take(3).reduce(|x,y|x+y).unwrap();
    return result
}


pub fn part_2() -> i64 {
    let inventory = read_file_as_vec("src/input/day01.txt".to_string());
    let res = sort_by_most_calories(inventory);
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day1_1_get_calories() {
        assert_eq!(0, get_calories(&vec![]));
        assert_eq!(1, get_calories(&vec![1]));
        assert_eq!(6, get_calories(&vec![1, 2, 3]));
    }

    #[test]
    fn test_day1_1_get_most_calories() {
        assert_eq!(
            7,
            get_most_calories(vec![vec![0, 7], vec![1, 2, 3], vec![1]])
        );
        assert_eq!(6, get_most_calories(vec![vec![0], vec![1, 2, 3], vec![1]]));
    }

    #[test]
    fn test_day1_1_input() {
        let inventory =  read_file_as_vec("src/input/day01.txt".to_string());
        let res = get_most_calories(inventory);
        assert_eq!(75622, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day1_2_sort_by_most_calories() {
        assert_eq!(
            21,
            sort_by_most_calories(vec![vec![0, 7], vec![1, 2, 3], vec![1], vec![3], vec![8]])
        );
    }

    #[test]
    fn test_day1_2_input() {
        let inventory =  read_file_as_vec("src/input/day01.txt".to_string());
        let res = sort_by_most_calories(inventory);
        assert_eq!(0, res);
    }
}
