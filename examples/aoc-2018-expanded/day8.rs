type Data = usize;

struct Node {
    children: Vec<Node>,
    metadata: Vec<Data>,
}

impl Node {
    fn from_iter<I>(iter: &mut I) -> Option<Node>
    where
        I: Iterator<Item = Data>,
    {
        let nb_children = iter.next()?;
        let nb_metadata = iter.next()?;

        let mut children = Vec::with_capacity(nb_children);
        for _ in 0..nb_children {
            children.push(Node::from_iter(iter)?);
        }

        let metadata = iter.take(nb_metadata).collect();

        Some(Node { children, metadata })
    }

    fn checksum(&self) -> Data {
        let children_sum: Data = self.children.iter().map(|c| c.checksum()).sum();
        let self_sum: Data = self.metadata.iter().sum();

        children_sum + self_sum
    }

    fn value(&self) -> Data {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| match i {
                    0 => None,
                    _ => self.children.get(i - 1).map(|c| c.value()),
                })
                .sum()
        }
    }
}

fn parse(input: &str) -> Option<Node> {
    Node::from_iter(&mut input.split_whitespace().map(|s| s.parse().unwrap()))
}

fn part1(root: &Node) -> Data {
    root.checksum()
}

mod day8_part1 {
    use super::*;
    use crate::{Day8Part1, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day8Part1 for Factory {
        fn day8_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Node,
        output: PhantomData<Data>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse(input.borrow()).ok_or("generator produce no value")?,
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

fn part2(root: &Node) -> Data {
    root.value()
}

mod day8_part2 {
    use super::*;
    use crate::{Day8Part2, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day8Part2 for Factory {
        fn day8_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Node,
        output: PhantomData<Data>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse(input.borrow()).ok_or("generator produce no value")?,
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1_example() {
        let root = parse(INPUT).unwrap();

        assert_eq!(part1(&root), 138);
    }

    #[test]
    fn part2_example() {
        let root = parse(INPUT).unwrap();

        assert_eq!(part2(&root), 66);
    }
}
