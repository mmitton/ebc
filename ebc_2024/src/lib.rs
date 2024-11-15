use helper::NewRunner;
use std::collections::BTreeMap;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

pub fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {
    runners.insert((2024, 1), (3, || Box::new(day_01::Day01::new())));
    runners.insert((2024, 2), (3, || Box::new(day_02::Day02::new())));
    runners.insert((2024, 3), (3, || Box::new(day_03::Day03::new())));
    runners.insert((2024, 4), (3, || Box::new(day_04::Day04::new())));
    runners.insert((2024, 5), (3, || Box::new(day_05::Day05::new())));
    runners.insert((2024, 6), (3, || Box::new(day_06::Day06::new())));
    runners.insert((2024, 7), (3, || Box::new(day_07::Day07::new())));
    runners.insert((2024, 8), (3, || Box::new(day_08::Day08::new())));
    runners.insert((2024, 9), (3, || Box::new(day_09::Day09::new())));
    runners.insert((2024, 10), (3, || Box::new(day_10::Day10::new())));
    runners.insert((2024, 11), (3, || Box::new(day_11::Day11::new())));
    runners.insert((2024, 12), (3, || Box::new(day_12::Day12::new())));
    runners.insert((2024, 13), (3, || Box::new(day_13::Day13::new())));
    runners.insert((2024, 14), (3, || Box::new(day_14::Day14::new())));
    runners.insert((2024, 15), (3, || Box::new(day_15::Day15::new())));
    runners.insert((2024, 16), (3, || Box::new(day_16::Day16::new())));
    runners.insert((2024, 17), (3, || Box::new(day_17::Day17::new())));
    runners.insert((2024, 18), (3, || Box::new(day_18::Day18::new())));
    runners.insert((2024, 19), (3, || Box::new(day_19::Day19::new())));
    runners.insert((2024, 20), (3, || Box::new(day_20::Day20::new())));
}
