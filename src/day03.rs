#![allow(dead_code)]

use std::fs::File;
use std::ops::Index;
use std::result::Result;
use std::string::ToString;
use itertools::Itertools;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let inventory = helpers::read(File::open("src/input/day03.txt").unwrap()).unwrap();
    let res = inventory
        .into_iter()
        .map(|backpack| get_item_value(get_identical_item(split_items(backpack))))
        .reduce(|a, e| a + e)
        .unwrap();
    return res;
}

fn split_items(items: String) -> (String, String) {
    let middle = items.len() / 2;
    let (first, second) = items.split_at(middle);
    return (first.to_string(), second.to_string());
}

fn get_identical_item((first, second): (String, String)) -> char {
    first.chars().find(|&c| second.contains(c)).unwrap()
}

const STRING_INDEX: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_item_value(c: char) -> i64 {
    STRING_INDEX.find(c).unwrap() as i64
}

// ____________________
// Part 2
// ____________________

pub fn part_2() -> i64 {
    let inventory = helpers::read(File::open("src/input/day03.txt").unwrap()).unwrap();
    let res = inventory.into_iter().tuples::<(_, _, _)>().map(|group| get_item_value(find_common_item(group)))
        .reduce(|a, e| a + e)
        .unwrap();
    return res;
}

fn find_common_item((first, second, third): (String, String, String)) -> char {
    first.chars().find(|&c| second.contains(c) && third.contains(c)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day3_1_split_items() {
        assert_eq!(
            ("aB".to_string(), "AB".to_string()),
            split_items("aBAB".to_string())
        );
    }

    #[test]
    fn test_day3_1_get_identical_item() {
        assert_eq!(
            'B',
            get_identical_item(("aB".to_string(), "AB".to_string()))
        );
        assert_eq!(
            'e',
            get_identical_item(("kjveRET".to_string(), "poewUCM".to_string()))
        );
    }

    #[test]
    fn test_day3_1_get_item_value() {
        assert_eq!(1, get_item_value('a'));
        assert_eq!(26, get_item_value('z'));
        assert_eq!(27, get_item_value('A'));
        assert_eq!(52, get_item_value('Z'));
    }

    #[test]
    fn test_day3_1_input() {
        let res = part_1();
        assert_eq!(8515, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day3_2_find_common_item() {
        assert_eq!('c', find_common_item(("abc".to_string(), "Abc".to_string(), "ABc".to_string())));
    }

    #[test]
    fn test_day3_2_sample() {
        let res = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ].into_iter().tuples::<(_, _, _)>().map(|group| get_item_value(find_common_item(group)))
            .reduce(|a, e| a + e)
            .unwrap();
        assert_eq!(70, res);
    }

    #[test]
    fn test_day3_2_input() {
        let res = part_2();
        assert_eq!(2434, res);
    }
}
