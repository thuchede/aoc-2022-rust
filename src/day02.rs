#![allow(dead_code)]

use std::fs::File;
use std::result::Result;
use std::str::FromStr;
use regex::Regex;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
    let res = inventory.into_iter().map(|strategy| get_point(parse_strategy(strategy))).reduce(|acc, el| acc+el).unwrap();
    return res
}

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "A" => Ok(Move::Rock),
            "X" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "Y" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

fn parse_strategy(line: String) -> (Move, Move) {
    let re = Regex::new(r"([ABCXYZ]) ([ABCXYZ])").unwrap();
    let groups = re.captures(&line).unwrap();
    let opponent_move = Move::from_str(groups.get(1).unwrap().as_str()).unwrap();
    let strategy_move = Move::from_str(groups.get(2).unwrap().as_str()).unwrap();
    return (opponent_move, strategy_move);
}

fn get_point(compare:(Move, Move)) -> i64 {
    let res = match compare.clone() {
        (a, b) if a == b => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        _ => unreachable!(),
    };
    return res + compare.1 as i64
}

// ____________________
// Part 2
// ____________________


pub fn part_2() -> i64 {
    let inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
    return 0
}


#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day2_1_parse_strategy() {
        let res = parse_strategy("A Y".to_string());
        assert_eq!((Move::Rock, Move::Paper), res);
    }

    #[test]
    fn test_day2_1_get_point() {
        assert_eq!(8, get_point((Move::Rock, Move::Paper)));
        assert_eq!(1, get_point((Move::Paper, Move::Rock)));
        assert_eq!(6, get_point((Move::Scissors, Move::Scissors)));
    }

    #[test]
    fn test_day2_1_input() {
        let _inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
        let res = 0;
        assert_eq!(75622, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day2_2_sort_by_most_calories() {
        assert_eq!(
            0,
            0
        );
    }

    #[test]
    fn test_day2_2_input() {
        let _inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
        let res = 0;
        assert_eq!(0, res);
    }
}
