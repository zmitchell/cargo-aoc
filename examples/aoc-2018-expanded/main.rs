mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub use self::aoc_factory::*;

mod aoc_factory {
    use aoc_runner::{ArcStr, Runner};
    use std::error::Error;

    pub static YEAR: u32 = 2018u32;
    pub struct Factory();

    pub trait Day7Part2 {
        fn day7_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day2Part1FNV {
        fn day2_part1_fnv(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day4Part1 {
        fn day4_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day3Part1 {
        fn day3_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day1Part1 {
        fn day1_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day5Part2STACK {
        fn day5_part2_stack(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day5Part2 {
        fn day5_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day8Part2 {
        fn day8_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day2Part1 {
        fn day2_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day2Part2 {
        fn day2_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day1Part2 {
        fn day1_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day3Part2 {
        fn day3_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day5Part1 {
        fn day5_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day1Part2FNV {
        fn day1_part2_fnv(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day4Part2 {
        fn day4_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day5Part1STACK {
        fn day5_part1_stack(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day6Part2 {
        fn day6_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day7Part1 {
        fn day7_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day8Part1 {
        fn day8_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }

    pub trait Day6Part1 {
        fn day6_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>>;
    }
}

fn main() {
    use aoc_runner::ArcStr;
    use std::time::Instant;

    let input_day1 = ArcStr::from(include_str!("../input/2018/day1.txt"));
    let input_day2 = ArcStr::from(include_str!("../input/2018/day2.txt"));

    println!("Advent of code {}\n", YEAR);
    {
        let start_time = Instant::now();
        match Factory::day1_part1(input_day1.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 1 - Part 1: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 1 - Part 1: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!("Day 1 - Part 1: FAILED while generating:\n{:#?}\n\n", e,);
            }
        }
    }
    {
        let start_time = Instant::now();
        match Factory::day1_part2(input_day1.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 1 - Part 2: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 1 - Part 2: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!("Day 1 - Part 2: FAILED while generating:\n{:#?}\n\n", e,);
            }
        }
    }
    {
        let start_time = Instant::now();
        match Factory::day1_part2_fnv(input_day1.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 1 - Part 2 - Fnv: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 1 - Part 2 - Fnv: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Day 1 - Part 2 - Fnv: FAILED while generating:\n{:#?}\n\n",
                    e,
                );
            }
        }
    }
    {
        let start_time = Instant::now();
        match Factory::day2_part1(input_day2.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 2 - Part 1: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 2 - Part 1: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!("Day 2 - Part 1: FAILED while generating:\n{:#?}\n\n", e,);
            }
        }
    }
    {
        let start_time = Instant::now();
        match Factory::day2_part1_fnv(input_day2.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 2 - Part 1 - Fnv: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 2 - Part 1 - Fnv: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Day 2 - Part 1 - Fnv: FAILED while generating:\n{:#?}\n\n",
                    e,
                );
            }
        }
    }
    {
        let start_time = Instant::now();
        match Factory::day2_part2(input_day2.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();
                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!(
                            "Day 2 - Part 2: {}\n\tgenerator: {:?},\n\trunner: {:?}\n\n",
                            result,
                            (inter_time - start_time),
                            (final_time - inter_time),
                        );
                    }
                    Err(e) => {
                        eprintln!("Day 2 - Part 2: FAILED while running:\n{:#?}\n\n", e,);
                    }
                }
            }
            Err(e) => {
                eprintln!("Day 2 - Part 2: FAILED while generating:\n{:#?}\n\n", e,);
            }
        }
    }
    // And so on...
}
