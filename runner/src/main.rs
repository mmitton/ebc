use helper::{Error, NewRunner};
use std::collections::BTreeMap;

const README_HEADER: &str = "[Everybody Codes](https://everybody.codes/)
Michael Conrad

[Helper library](https://github.com/mmitton/helper) which holds generic algorithms and runner
infrastructure shared between AOC and Everybody Codes";

fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {
    ebc_2024::register(runners);
}

fn today(year: usize, month: usize, day: usize) -> (usize, usize) {
    let days = &[
        (5, 1),
        (6, 2),
        (7, 3),
        (8, 4),
        (9, 5),
        (12, 6),
        (13, 7),
        (14, 8),
        (15, 9),
        (16, 10),
        (19, 11),
        (20, 12),
        (21, 13),
        (22, 14),
        (23, 15),
        (26, 16),
        (27, 17),
        (28, 18),
        (29, 19),
        (30, 20),
    ];

    use std::cmp::Ordering;
    match month.cmp(&11) {
        Ordering::Less => (year - 1, 20),
        Ordering::Equal => {
            if let Some((_, day)) = days.iter().filter(|(d, _)| *d <= day).last() {
                (year, *day)
            } else {
                (year - 1, 20)
            }
        }
        Ordering::Greater => (year, 20),
    }
}

fn main() -> Result<(), Error> {
    let mut config = helper::runner::Config::new(register, today);
    config.download_input(false);
    config.allow_copy(false);
    config.readme_header(README_HEADER);
    helper::runner::main::<_, _, 3>(config)
}
