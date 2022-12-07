#![allow(dead_code)]

use itertools::Itertools;
use std::fs::File;
use std::result::Result;

use crate::helpers;

// ____________________
// Part 1
// ____________________

pub fn part_1() -> i64 {
    let signals = helpers::read(File::open("src/input/day06.txt").unwrap()).unwrap();
    let res = get_marker(signals.get(0).unwrap().into());
    return res;
}

fn get_marker(signal: String) -> i64 {
    let packets = signal.chars().collect::<Vec<char>>();
    let index_of_marker = packets
        .windows(4)
        .enumerate()
        .find(|(idx, packet)| packet.iter().all_unique())
        .unwrap();
    return (index_of_marker.0 + 4) as i64;
}

// ____________________
// Part 2
// ____________________

pub fn part_2() -> i64 {
    let signals = helpers::read(File::open("src/input/day06.txt").unwrap()).unwrap();
    let res = get_message_marker(signals.get(0).unwrap().into());
    return res;
}

fn get_message_marker(signal: String) -> i64 {
    let window = 14;
    let packets = signal.chars().collect::<Vec<char>>();
    let index_of_marker = packets
        .windows(window)
        .enumerate()
        .find(|(idx, packet)| packet.iter().all_unique())
        .unwrap();
    return (index_of_marker.0 + window) as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ____________________
    // Part 1
    // ____________________

    #[test]
    fn test_day1_1_get_marker() {
        assert_eq!(7, get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!(5, get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(6, get_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(
            10,
            get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            11,
            get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }

    #[test]
    fn test_day1_1_input() {
        let res = part_1();
        assert_eq!(1356, res);
    }

    // ____________________
    // Part 2
    // ____________________

    #[test]
    fn test_day1_2_get_message_marker() {
        assert_eq!(
            19,
            get_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())
        );
        assert_eq!(
            23,
            get_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())
        );
        assert_eq!(
            23,
            get_message_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string())
        );
        assert_eq!(
            29,
            get_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            26,
            get_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }

    #[test]
    fn test_day1_2_input() {
        let res = part_2();
        assert_eq!(2564, res);
    }
}
