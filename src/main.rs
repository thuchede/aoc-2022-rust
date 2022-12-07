#![allow(dead_code)]
extern crate regex;

use std::time::{Duration, Instant};

mod helpers;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let now = Instant::now();
    println!("Part - 1 : {:?}", day04::part_1());
    println!("{}", now.elapsed().as_millis());

    let now2 = Instant::now();
    println!("Part - 2 : {:?}", day04::part_2());
    println!("{}", now2.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        let value = true;
        assert_eq!(true, value);
    }

    #[test]
    fn test_day1() {
        let value = false;
        assert_eq!(false, value);
    }
}