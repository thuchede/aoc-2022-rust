#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::result::Result;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let inventory = helpers::read(File::open("src/input/day04.txt").unwrap()).unwrap();
    let res = inventory
        .into_iter()
        .map(|ranges| parse_range(ranges))
        .filter(|&ranges| is_overlapping(ranges))
        .collect::<Vec<_>>()
        .len() as i64;
    return res;
}

fn parse_range(ranges: String) -> ((i64, i64), (i64, i64)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let groups = RE.captures(&ranges).unwrap();
    let first_start = groups.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let first_end = groups.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let second_start = groups.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let second_end = groups.get(4).unwrap().as_str().parse::<i64>().unwrap();
    return ((first_start, first_end), (second_start, second_end));
}

fn is_overlapping(
    ((first_start, first_end), (second_start, second_end)): ((i64, i64), (i64, i64)),
) -> bool {
    first_start <= second_start && first_end >= second_end
        || first_start >= second_start && first_end <= second_end
}

// ____________________
// Part 2
// ____________________

pub fn part_2() -> i64 {
    let inventory = helpers::read(File::open("src/input/day04.txt").unwrap()).unwrap();
    let res = inventory
        .into_iter()
        .map(|ranges| parse_range(ranges))
        .filter(|&ranges| is_overlapping_partial(ranges))
        .collect::<Vec<_>>()
        .len() as i64;
    return res;
}

fn is_overlapping_partial(
    ((first_start, first_end), (second_start, second_end)): ((i64, i64), (i64, i64)),
) -> bool {
    second_start <= first_end && second_end >= first_start
        || first_start <= second_end && first_end >= second_start
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day1_1_parse_range() {
        assert_eq!(((1, 2), (3, 4)), parse_range("1-2,3-4".to_string()));
        assert_eq!(((12, 24), (33, 40)), parse_range("12-24,33-40".to_string()));
    }

    #[test]
    fn test_day1_1_is_overlapping() {
        assert_eq!(false, is_overlapping(((1, 2), (3, 4))));
        assert_eq!(true, is_overlapping(((12, 24), (13, 20))));
        assert_eq!(true, is_overlapping(((12, 24), (23, 24))));
        assert_eq!(true, is_overlapping(((12, 24), (12, 12))));
    }

    #[test]
    fn test_day1_1_input() {
        let res = part_1();
        assert_eq!(567, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day1_1_is_overlapping_partial() {
        assert_eq!(false, is_overlapping_partial(((1, 2), (3, 4))));
        assert_eq!(true, is_overlapping_partial(((12, 24), (13, 25))));
        assert_eq!(true, is_overlapping_partial(((12, 24), (23, 23))));
        assert_eq!(true, is_overlapping_partial(((12, 24), (11, 12))));
    }

    #[test]
    fn test_day1_2_input() {
        let res = part_2();
        assert_eq!(907, res);
    }
}
