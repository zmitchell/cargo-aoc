use fnv::FnvHashMap;
use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let (nb_double, nb_triple) = input
        .lines()
        .map(|l| {
            let mut map = HashMap::with_capacity(l.len());

            l.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

            let twice = map.values().any(|&n| n == 2);
            let three_times = map.values().any(|&n| n == 3);

            (twice, three_times)
        })
        .fold((0, 0), |acc, n| match n {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        });

    nb_double * nb_triple
}
mod day2_part1 {
    use super::*;
    use crate::{Day2Part1, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day2Part1 for Factory {
        fn day2_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<u32>,
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

fn part1_fnv(input: &str) -> u32 {
    let (nb_double, nb_triple) = input
        .lines()
        .map(|l| {
            let mut map = FnvHashMap::default();
            map.reserve(l.len());

            l.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

            let twice = map.values().any(|&n| n == 2);
            let three_times = map.values().any(|&n| n == 3);

            (twice, three_times)
        })
        .fold((0, 0), |acc, n| match n {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        });

    nb_double * nb_triple
}

mod day2_part1_fnv {
    use super::*;
    use crate::{Day2Part1FNV, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day2Part1FNV for Factory {
        fn day2_part1_fnv(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<u32>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part1_fnv(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part1_fnv(self.input.borrow()))
        }
    }
}

fn part2(input: &str) -> String {
    let lines = input.lines();

    for (i, l1) in lines.clone().enumerate() {
        for (_, l2) in lines.clone().enumerate().filter(|&(j, _)| i != j) {
            let filtered: String = l1
                .chars()
                .zip(l2.chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect();

            if filtered.len() == l1.len() - 1 {
                return filtered;
            }
        }
    }

    unreachable!()
}

mod day2_part2 {
    use super::*;
    use crate::{Day2Part2, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day2Part2 for Factory {
        fn day2_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: ArcStr,
        output: PhantomData<String>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            RunnerStruct {
                input,
                output: PhantomData,
            }
        }
        fn run(&self) -> Box<dyn Display> {
            Box::new(part2(self.input.borrow()))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part2(self.input.borrow()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const INPUT_PART2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_PART1), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_PART2), "fgij");
    }
}
