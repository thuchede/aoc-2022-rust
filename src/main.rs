#![allow(dead_code)]
extern crate core;
extern crate regex;

use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod helpers;

fn main() {
    let now = Instant::now();
    println!("Part - 1 : {:?}", day06::part_1());
    println!("{}", now.elapsed().as_millis());

    let now2 = Instant::now();
    println!("Part - 2 : {:?}", day06::part_2());
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
