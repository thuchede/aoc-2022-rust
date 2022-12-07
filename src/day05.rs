#![allow(dead_code)]

use regex::Regex;
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::fs::File;
use std::result::Result;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> String {
    let input = helpers::read(File::open("src/input/day05.txt").unwrap()).unwrap();
    let instructions: Vec<Vec<String>> = input
        .split(|line| *line == "")
        .map(|e| e.into_iter().map(|s| s.clone()).collect::<Vec<String>>())
        .collect();
    let drawing: Vec<String> = instructions.get(0).unwrap().to_owned();
    let procedure_txt: Vec<String> = instructions.get(1).unwrap().to_owned();
    let initial_stacks = get_stacks_2(drawing);

    let procedure: Vec<(i64, i64, i64)> = get_procedure(procedure_txt);

    let final_arrangement =
        procedure
            .into_iter()
            .fold(initial_stacks, |mut stacks, (count, from, to)| {
                for _ in 0..count {
                    let elm = stacks[from as usize - 1].pop().unwrap();
                    stacks[to as usize - 1].push(elm); // TODO: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off for P2?
                }
                stacks
            });

    let containers_on_top = final_arrangement
        .iter()
        .fold("".to_string(), |cur, nxt| cur + nxt.last().unwrap());
    return containers_on_top;
}

fn get_stacks_2(drawing: Vec<String>) -> Vec<Vec<String>> {
    let mut copy = drawing.clone();
    let cols = copy.pop().unwrap();
    let re = Regex::new(r"\d").unwrap();
    let groups: Vec<_> = re.captures_iter(&cols).collect();
    let mut stacks: Vec<Vec<String>> = vec![vec![]; groups.len()];
    copy.into_iter().rev().for_each(|level| {
        level
            .char_indices()
            .skip(1)
            .step_by(4)
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i / 4].push(c.to_string()))
    });
    return stacks;
}

fn get_stacks(drawing: Vec<String>) -> Vec<VecDeque<String>> {
    let mut copy = drawing.clone();
    let cols = copy.pop().unwrap();
    let re = Regex::new(r"\d").unwrap();
    let groups: Vec<_> = re.captures_iter(&cols).collect();
    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::from(vec![]); groups.len()];
    copy.into_iter().for_each(|level| {
        level
            .char_indices()
            .skip(1)
            .step_by(4)
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i / 4].push_front(c.to_string()))
    });
    return stacks;
}

fn get_procedure(procedure: Vec<String>) -> Vec<(i64, i64, i64)> {
    let mut steps = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    procedure.iter().for_each(|step| {
        let groups = re.captures(step).unwrap();
        let container = groups
            .get(1)
            .unwrap()
            .as_str()
            .to_owned()
            .parse::<i64>()
            .unwrap();
        let from = groups
            .get(2)
            .unwrap()
            .as_str()
            .to_owned()
            .parse::<i64>()
            .unwrap();
        let to = groups
            .get(3)
            .unwrap()
            .as_str()
            .to_owned()
            .parse::<i64>()
            .unwrap();
        steps.push((container, from, to))
    });
    return steps;
}

// ____________________
// Part 2
// ____________________

pub fn part_2() -> String {
    let input = helpers::read(File::open("src/input/day05.txt").unwrap()).unwrap();
    let instructions: Vec<Vec<String>> = input
        .split(|line| *line == "")
        .map(|e| e.into_iter().map(|s| s.clone()).collect::<Vec<String>>())
        .collect();
    let drawing: Vec<String> = instructions.get(0).unwrap().to_owned();
    let procedure_txt: Vec<String> = instructions.get(1).unwrap().to_owned();
    let initial_stacks = get_stacks_2(drawing);

    let procedure: Vec<(i64, i64, i64)> = get_procedure(procedure_txt);

    let final_arrangement =
        procedure
            .into_iter()
            .fold(initial_stacks, |mut stacks, (count, from, to)| {
                // let mut pile = stacks[from as usize - 1];
                let len = stacks[from as usize - 1].len();
                let grabbed = stacks[from as usize - 1].split_off(len - count as usize);
                stacks[to as usize - 1].extend(grabbed);
                stacks
            });

    let containers_on_top = final_arrangement
        .iter()
        .fold("".to_string(), |cur, nxt| cur + nxt.last().unwrap());
    return containers_on_top;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day1_1_get_stacks() {
        assert_eq!(
            vec![
                VecDeque::from(vec!["A".to_string()]),
                VecDeque::from(vec!["B".to_string()]),
                VecDeque::from(vec!["C".to_string()]),
            ],
            get_stacks(vec!["[A] [B] [C]".to_string(), " 1   2   3 ".to_string()])
        );
        assert_eq!(
            vec![
                VecDeque::from(vec!["A".to_string(), "D".to_string()]),
                VecDeque::from(vec!["B".to_string()]),
                VecDeque::from(vec!["C".to_string(), "E".to_string()]),
            ],
            get_stacks(vec![
                "[D]     [E]".to_string(),
                "[A] [B] [C]".to_string(),
                " 1   2   3 ".to_string(),
            ])
        );
    }

    #[test]
    fn test_day1_1_get_stacks_2() {
        assert_eq!(
            vec![
                vec!["A".to_string()],
                vec!["B".to_string()],
                vec!["C".to_string()],
            ],
            get_stacks_2(vec!["[A] [B] [C]".to_string(), " 1   2   3 ".to_string()])
        );
        assert_eq!(
            vec![
                vec!["A".to_string(), "D".to_string()],
                vec!["B".to_string()],
                vec!["C".to_string(), "E".to_string()],
            ],
            get_stacks_2(vec![
                "[D]     [E]".to_string(),
                "[A] [B] [C]".to_string(),
                " 1   2   3 ".to_string(),
            ])
        );
    }

    #[test]
    fn test_day1_1_get_procedure() {
        assert_eq!(
            vec![(1, 2, 3)],
            get_procedure(vec!["move 1 from 2 to 3".to_string()])
        );
        assert_eq!(
            vec![(1, 2, 3), (1, 7, 9)],
            get_procedure(vec![
                "move 1 from 2 to 3".to_string(),
                "move 1 from 7 to 9".to_string(),
            ])
        );
    }

    #[test]
    fn test_day1_1_input() {
        let res = part_1();
        assert_eq!("GRTSWNJHH".to_string(), res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day1_2_input() {
        let res = part_2();
        assert_eq!("QLFQDBBHM".to_string(), res);
    }
}
