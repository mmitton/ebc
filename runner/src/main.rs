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
        (4, 1),
        (5, 2),
        (6, 3),
        (7, 4),
        (8, 5),
        (11, 6),
        (12, 7),
        (13, 8),
        (14, 9),
        (15, 10),
        (18, 11),
        (19, 12),
        (20, 13),
        (21, 14),
        (22, 15),
        (25, 16),
        (26, 17),
        (27, 18),
        (28, 19),
        (29, 20),
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
    config.readme_header(README_HEADER);
    helper::runner::main::<_, _, 3>(config)
}
