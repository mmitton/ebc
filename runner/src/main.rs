use helper::{Error, InputFileCache};
use std::{cmp::Ordering, collections::BTreeMap, time::Duration};

mod args;
mod run;

struct TimesCacheEntry {
    day: usize,
    results: BTreeMap<u8, Result<Duration, Error>>,
}

fn print_times(
    md: bool,
    run_count: usize,
    parts: u8,
    times_cache: &BTreeMap<usize, Vec<TimesCacheEntry>>,
) {
    fn print_dashed(parts: u8, header: &str) {
        fn dashed(len: usize) {
            for _ in 0..len {
                print!("-");
            }
        }
        print!("+");
        dashed(header.len() + 2);
        print!("+");
        for _ in 1..=parts {
            dashed(12);
            print!("+");
        }
        if header == "Year" {
            dashed(13);
            println!("+");
        } else {
            for _ in 1..=parts {
                dashed(10);
                print!("+");
            }
            println!();
        }
    }
    fn print_header(md: bool, parts: u8, header: &str) {
        if md {
            print!("| {header} |");
            for part in 1..=parts {
                print!(" Part {part} |");
            }
            if header == "Year" {
                println!(" Total |");
            } else {
                for part in 1..=parts {
                    print!(" Part {part} % |");
                }
                println!();
            }
            print!("| ---: |");
            for _ in 1..=parts {
                print!(" --: |");
            }
            if header == "Year" {
                println!(" ---: |");
            } else {
                for _ in 1..=parts {
                    print!(" --: |");
                }
                println!();
            }
        } else {
            print_dashed(parts, header);
            print!("| {header} |");
            for part in 1..=parts {
                print!(" {part:>10} |", part = format!("Part {part}"));
            }
            if header == "Year" {
                println!(" {total:>11} |", total = "Total");
            } else {
                for part in 1..=parts {
                    print!(" {part:>8} |", part = format!("Part {part} %"));
                }
                println!();
            }
            print_dashed(parts, header);
        }
    }
    if times_cache.len() > 1 {
        print_header(md, parts, "Year");
        for (year, times_cache) in times_cache.iter().rev() {
            let mut total = Duration::new(0, 0);
            let mut part_totals: BTreeMap<u8, Duration> = BTreeMap::new();
            for entry in times_cache.iter() {
                for (part, result) in entry.results.iter() {
                    if let Ok(dur) = result {
                        *part_totals.entry(*part).or_default() += *dur;
                        total += *dur;
                    }
                }
            }
            if md {
                print!("| {year} |");
                for part in 1..=parts {
                    if let Some(dur) = part_totals.get(&part) {
                        print!(" {dur:0.5} s |", dur = dur.as_secs_f64());
                    } else {
                        print!(" |");
                    }
                }
                println!(" {dur:0.5} s |", dur = total.as_secs_f64());
            } else {
                print!("| {year} |");
                for part in 1..=parts {
                    if let Some(dur) = part_totals.get(&part) {
                        print!(" {dur:>10} |", dur = format!("{:0.5} s", dur.as_secs_f64()));
                    } else {
                        print!(" |");
                    }
                }
                println!(
                    " {dur:>11} |",
                    dur = format!("{:0.5} s", total.as_secs_f64())
                );
            }
        }
        if !md {
            print_dashed(parts, "Year")
        }
        println!();
    }

    for (year, times_cache) in times_cache.iter().rev() {
        let mut total = Duration::new(0, 0);
        let mut part_totals: BTreeMap<u8, Duration> = BTreeMap::new();
        for entry in times_cache.iter() {
            for (part, result) in entry.results.iter() {
                if let Ok(dur) = result {
                    *part_totals.entry(*part).or_default() += *dur;
                    total += *dur;
                }
            }
        }

        if run_count > 1 {
            println!("Year: {year}  Averaged over {run_count} runs.");
        } else {
            println!("Year: {year}");
        }
        print_header(md, parts, "Day");
        for TimesCacheEntry { day, results } in times_cache.iter().rev() {
            if !results.values().any(|result| result.is_ok()) {
                continue;
            }
            if md {
                print!("| {day} |");
            } else {
                print!("| {day:>3} |");
            }
            for part in 1..=parts {
                let time = if let Some(Ok(dur)) = results.get(&part) {
                    format!("{:0.5} s", dur.as_secs_f64())
                } else {
                    String::new()
                };
                if md {
                    print!(" {time} |");
                } else {
                    print!(" {time:>10} |");
                }
            }
            for part in 1..=parts {
                let percent = if let Some(Ok(dur)) = results.get(&part) {
                    format!("{:0.2}%", dur.as_secs_f64() / total.as_secs_f64() * 100.)
                } else {
                    String::new()
                };
                if md {
                    print!(" {percent} |");
                } else {
                    print!(" {percent:>8} |");
                }
            }
            println!();
        }

        if !md {
            print_dashed(parts, "Day");
        }
        print!("| All |");
        for part in 1..=parts {
            let time = if let Some(dur) = part_totals.get(&part) {
                format!("{:0.5} s", dur.as_secs_f64())
            } else {
                String::new()
            };
            if md {
                print!(" {time} |");
            } else {
                print!(" {time:>10} |");
            }
        }
        for part in 1..=parts {
            let percent = if let Some(dur) = part_totals.get(&part) {
                format!("{:0.2}%", dur.as_secs_f64() / total.as_secs_f64() * 100.)
            } else {
                String::new()
            };
            if md {
                print!(" {percent} |");
            } else {
                print!(" {percent:>8} |");
            }
        }
        println!();

        if !md {
            print_dashed(parts, "Day");
        }
        println!();
    }
}

fn main() -> Result<(), Error> {
    let (sample_data, no_capture, times, md, target_year, target_day) = args::get();

    let mut runners = BTreeMap::new();
    ebc_2024::register(&mut runners);

    if times.is_some() {
        helper::output(|output| output.no_output());
    } else if cfg!(debug_assertions) || no_capture {
        helper::output(|output| output.stdout());
    } else {
        helper::output(|output| output.capture());
    }

    use chrono::prelude::*;
    let today = Local::now();

    let mut times_cache: BTreeMap<usize, Vec<TimesCacheEntry>> = BTreeMap::new();
    let run_count = times.unwrap_or(1);

    let input_file_cache: InputFileCache<3> = helper::InputFileCache::new()?;
    for ((year, day), (parts, new_runner)) in runners.iter().rev() {
        if let Some(target_year) = target_year {
            if target_year != *year {
                continue;
            }
        }
        if let Some(target_day) = target_day {
            if target_day != *day {
                continue;
            }
        }

        match (
            (today.year() as usize).cmp(year),
            (today.month() as usize).cmp(&11),
            (today.day() as usize).cmp(day),
        ) {
            (Ordering::Less, _, _) => continue,
            (Ordering::Equal, Ordering::Less, _) => continue,
            (Ordering::Equal, Ordering::Equal, Ordering::Less) => continue,
            _ => {}
        }

        let mut times_cache_entry = TimesCacheEntry {
            day: *day,
            results: BTreeMap::new(),
        };
        for part in 1..=*parts {
            let result = run::run(
                sample_data,
                new_runner,
                times.is_none(),
                run_count,
                *year,
                *day,
                1,
                &input_file_cache,
            );
            times_cache_entry.results.insert(part, result);
        }
        times_cache
            .entry(*year)
            .or_default()
            .push(times_cache_entry);
    }

    if times.is_some() && !times_cache.is_empty() {
        let parts = *runners.values().map(|(parts, _)| parts).max().unwrap();
        print_times(md, run_count, parts, &times_cache);
    }

    Ok(())
}
