#![allow(dead_code)]

use regex::Regex;
use std::fs::File;
use std::result::Result;
use std::str::FromStr;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
    let res = inventory
        .into_iter()
        .map(|strategy| get_point(parse_strategy(strategy)))
        .reduce(|acc, el| acc + el)
        .unwrap();
    return res;
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

fn get_point(compare: (Move, Move)) -> i64 {
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
    return res + compare.1 as i64;
}

// ____________________
// Part 2
// ____________________

#[derive(Debug, PartialEq, Clone)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Tie),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn parse_strategy_outcome(line: String) -> (Move, Outcome) {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let groups = re.captures(&line).unwrap();
    let opponent_move = Move::from_str(groups.get(1).unwrap().as_str()).unwrap();
    let strategy_outcome = Outcome::from_str(groups.get(2).unwrap().as_str()).unwrap();
    return (opponent_move, strategy_outcome);
}

fn get_point_outcome((opponent_move, outcome): (Move, Outcome)) -> i64 {
    let res = match outcome {
        Outcome::Lose => 0 + (opponent_move as i64 + 1) % 3 + 1,
        Outcome::Tie => 3 + opponent_move as i64,
        // Outcome::Win => 6 + (opponent_move as i64 -4)%3+3,
        Outcome::Win => 6 + (opponent_move as i64 % 3 + 1),
        _ => unreachable!(),
    };
    return res;
}

pub fn part_2() -> i64 {
    let inventory = helpers::read(File::open("src/input/day02.txt").unwrap()).unwrap();
    let res = inventory
        .into_iter()
        .map(|strategy| get_point_outcome(parse_strategy_outcome(strategy)))
        .reduce(|acc, el| acc + el)
        .unwrap();
    return res;
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
        let res = part_1();
        assert_eq!(11666, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day2_2_parse_strategy_outcome() {
        assert_eq!(
            (Move::Rock, Outcome::Lose),
            parse_strategy_outcome("A X".to_string())
        );
    }

    #[test]
    fn test_day2_1_get_point_outcome() {
        assert_eq!(4, get_point_outcome((Move::Rock, Outcome::Tie)));
        assert_eq!(1, get_point_outcome((Move::Paper, Outcome::Lose)));
        assert_eq!(7, get_point_outcome((Move::Scissors, Outcome::Win)));
    }

    #[test]
    fn test_day2_2_input() {
        let res = part_2();
        assert_eq!(12767, res);
    }
}
