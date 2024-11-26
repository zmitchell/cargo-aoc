use fnv::FnvHashSet;
use std::collections::HashSet;
use std::num::ParseIntError;

fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn part1(freqs: &[i32]) -> i32 {
    freqs.iter().sum()
}

mod day1_part1 {
    use super::*;
    use crate::{Day1Part1, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day1Part1 for Factory {
        fn day1_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Vec<i32>,
        output: PhantomData<i32>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse_input_day1(input.borrow())?,
                output: PhantomData,
            })
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part1(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part1(self.input.borrow()))
        }
    }
}

fn part2(freqs: &[i32]) -> i32 {
    let mut reached = HashSet::new();
    let mut sum = 0;

    reached.insert(sum);

    freqs
        .iter()
        .cycle()
        .take_while(|&&f| {
            sum += f;
            reached.insert(sum)
        })
        .count();

    sum
}

mod day1_part2 {
    use super::*;
    use crate::{Day1Part2, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day1Part2 for Factory {
        fn day1_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Vec<i32>,
        output: PhantomData<i32>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse_input_day1(input.borrow())?,
                output: PhantomData,
            })
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part2(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part2(self.input.borrow()))
        }
    }
}

fn part2_fnv(freqs: &[i32]) -> i32 {
    let mut reached = FnvHashSet::default();
    let mut sum = 0;

    reached.insert(sum);

    freqs
        .iter()
        .cycle()
        .take_while(|&&f| {
            sum += f;
            reached.insert(sum)
        })
        .count();

    sum
}

mod day1_part2_fnv {
    use super::*;
    use crate::{Day1Part2FNV, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day1Part2FNV for Factory {
        fn day1_part2_fnv(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Vec<i32>,
        output: PhantomData<i32>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse_input_day1(input.borrow())?,
                output: PhantomData,
            })
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part2_fnv(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part2_fnv(self.input.borrow()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1, -2, 3, 1]), 3);
        assert_eq!(part1(&[1, 1, 1]), 3);
        assert_eq!(part1(&[1, 1, -2]), 0);
        assert_eq!(part1(&[-1, -2, -3]), -6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1, -2, 3, 1]), 2);
        assert_eq!(part2(&[1, -1]), 0);
        assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
    }

    #[test]
    fn part2_fnv_example() {
        assert_eq!(part2_fnv(&[1, -2, 3, 1]), 2);
        assert_eq!(part2_fnv(&[1, -1]), 0);
        assert_eq!(part2_fnv(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2_fnv(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2_fnv(&[7, 7, -2, -7, -4]), 14);
    }
}
