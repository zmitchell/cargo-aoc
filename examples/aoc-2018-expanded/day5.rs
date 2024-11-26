use std::cell::Cell;
use std::num::NonZeroU8;

const DIFF: u8 = b'a' - b'A';

#[inline]
fn diff(a: u8, b: u8) -> u8 {
    u8::max(a, b) - u8::min(a, b)
}

fn part1(input: &[u8]) -> usize {
    reduce(input)
}

mod day5_part1 {
    use super::*;
    use crate::{Day5Part1, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day5Part1 for Factory {
        fn day5_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part1(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part1(self.input.borrow()))
        }
    }
}

fn part2(input: &[u8]) -> Option<usize> {
    (b'A'..=b'Z')
        .map(|c| reduce(input.iter().filter(|&&a| a != c && a != c + DIFF)))
        .min()
}
mod day5_part2 {
    use super::*;
    use crate::{Day5Part2, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day5Part2 for Factory {
        fn day5_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            self.try_run().expect("failed to run")
        }
        fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
            Ok(Box::new(
                part2(self.input.borrow()).ok_or("runner produce no value")?,
            ))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part2(self.input.borrow()).unwrap())
        }
    }
}

fn part1_stack(input: &[u8]) -> usize {
    stack(input)
}

mod day5_part1_stack {
    use super::*;
    use crate::{Day5Part1STACK, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day5Part1STACK for Factory {
        fn day5_part1_stack(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part1_stack(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part1_stack(self.input.borrow()))
        }
    }
}

fn part2_stack(input: &[u8]) -> Option<usize> {
    (b'A'..=b'Z')
        .map(|c| stack(input.iter().filter(|&&a| a != c && a != c + DIFF)))
        .min()
}
mod day5_part2_stack {
    use super::*;
    use crate::{Day5Part2STACK, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day5Part2STACK for Factory {
        fn day5_part2_stack(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            self.try_run().expect("failed to run")
        }
        fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
            Ok(Box::new(
                part2_stack(self.input.borrow()).ok_or("runner produce no value")?,
            ))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part2_stack(self.input.borrow()).unwrap())
        }
    }
}

fn reduce<'a>(polymer: impl IntoIterator<Item = &'a u8>) -> usize {
    let polymer: Vec<_> = polymer
        .into_iter()
        .map(|&a| Cell::new(NonZeroU8::new(a)))
        .collect();

    let mut i = 0;
    loop {
        if i + 1 >= polymer.len() {
            break;
        }

        let current = &polymer[i];
        let next = if let Some(b) = polymer[i + 1..].iter().find(|b| b.get().is_some()) {
            b
        } else {
            break;
        };

        match (current.get(), next.get()) {
            (Some(c), Some(n)) => {
                if diff(c.get(), n.get()) == DIFF {
                    current.set(None);
                    next.set(None);

                    i = polymer[..i]
                        .iter()
                        .enumerate()
                        .rev()
                        .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                        .unwrap_or_else(|| {
                            polymer
                                .iter()
                                .enumerate()
                                .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                                .unwrap()
                        })
                } else {
                    i = polymer
                        .iter()
                        .enumerate()
                        .skip(i + 1)
                        .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                        .unwrap();
                }
            }
            _ => panic!("{}\n{:#?}", i, polymer),
        }
    }

    polymer.into_iter().filter(|a| a.get().is_some()).count()
}

fn stack<'a>(polymer: impl IntoIterator<Item = &'a u8>) -> usize {
    polymer
        .into_iter()
        .fold(Vec::new(), |mut stack, &unit| {
            match stack.last() {
                Some(&other) if diff(other, unit) == DIFF => {
                    stack.pop();
                }
                _ => stack.push(unit),
            }

            stack
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(b"dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part1_stack(b"dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(b"dabAcCaCBAcCcaDA"), Some(4));
        assert_eq!(part2_stack(b"dabAcCaCBAcCcaDA"), Some(4));
    }
}
