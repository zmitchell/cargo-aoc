use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(self, other: Point) -> u32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Point, Box<dyn Error>> {
        let (x, y) = s.split_at(s.find(", ").ok_or("invalid input")?);

        Ok(Point {
            x: x.parse()?,
            y: y[2..].parse()?,
        })
    }
}

fn bounds(points: &[Point]) -> (Option<Point>, Option<Point>) {
    points.iter().fold((None, None), |(tl, br), &p| {
        (
            match tl {
                None => Some(p),
                Some(tl) => Some(Point {
                    x: p.x.min(tl.x),
                    y: p.y.min(tl.y),
                }),
            },
            match br {
                None => Some(p),
                Some(br) => Some(Point {
                    x: p.x.max(br.x),
                    y: p.y.max(br.y),
                }),
            },
        )
    })
}

fn all_points(tl: Point, br: Point) -> impl Iterator<Item = Point> {
    (tl.x..=br.x).flat_map(move |x| (tl.y..=br.y).map(move |y| Point { x, y }))
}

fn parse(input: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    input.lines().map(Point::from_str).collect()
}

fn part1(points: &[Point]) -> Option<usize> {
    let (tl, br) = bounds(points);

    let tl = tl?;
    let br = br?;

    let mut infinites = FnvHashSet::default();

    let areas = all_points(tl, br)
        .filter_map(|p| {
            let mut closest = None;
            let mut closest_indexs = Vec::new();

            for (index, dist) in points.iter().map(|&other| p.distance(other)).enumerate() {
                match closest {
                    None => {
                        closest = Some(dist);
                        closest_indexs.push(index);
                    }
                    Some(min_d) if dist < min_d => {
                        closest = Some(dist);
                        closest_indexs.clear();
                        closest_indexs.push(index);
                    }
                    Some(min_d) if dist == min_d => {
                        closest_indexs.push(index);
                    }
                    _ => (),
                }
            }

            if closest_indexs.len() == 1 {
                if p.x == tl.x || p.y == tl.y || p.x == br.x || p.y == br.y {
                    infinites.extend(closest_indexs.iter().cloned())
                }

                Some(closest_indexs[0])
            } else {
                None
            }
        })
        .fold(FnvHashMap::default(), |mut acc, i| {
            *acc.entry(i).or_default() += 1;
            acc
        });

    let max_area = areas
        .into_iter()
        .filter(|(i, _)| !infinites.contains(i))
        .max_by_key(|&(_, size)| size);

    max_area.map(|(_, size)| size)
}

mod day6_part1 {
    use super::*;
    use crate::{Day6Part1, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day6Part1 for Factory {
        fn day6_part1(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Vec<Point>,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse(input.borrow())?,
                output: PhantomData,
            })
        }
        fn run(&self) -> Box<dyn Display> {
            self.try_run().expect("failed to run")
        }
        fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
            Ok(Box::new(
                part1(self.input.borrow()).ok_or("runner produce no value")?,
            ))
        }
        fn bench(&self, black_box: fn(&dyn Display)) {
            black_box(&part1(self.input.borrow()).unwrap())
        }
    }
}

fn part2(points: &[Point]) -> Option<usize> {
    part2_internal(points, 10_000)
}

mod day6_part2 {
    use super::*;
    use crate::{Day6Part2, Factory};
    use aoc_runner::{ArcStr, Runner};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fmt::Display;
    use std::marker::PhantomData;
    impl Day6Part2 for Factory {
        fn day6_part2(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
            Ok(Box::new(RunnerStruct::try_gen(input)?))
        }
    }
    pub struct RunnerStruct {
        input: Vec<Point>,
        output: PhantomData<usize>,
    }
    impl Runner for RunnerStruct {
        fn gen(input: ArcStr) -> Self {
            Self::try_gen(input).expect("failed to generate input")
        }
        fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
            Ok(RunnerStruct {
                input: parse(input.borrow())?,
                output: PhantomData,
            })
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

fn part2_internal(points: &[Point], dist: u32) -> Option<usize> {
    let (tl, br) = bounds(points);

    let tl = tl?;
    let br = br?;

    Some(
        all_points(tl, br)
            .map(|p| points.iter().map(|&other| p.distance(other)).sum())
            .filter(|&s: &u32| s < dist)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";

    #[test]
    fn part1_example() {
        let points = parse(INPUT).unwrap();

        assert_eq!(part1(&points).unwrap(), 17);
    }

    #[test]
    fn part2_example() {
        let points = parse(INPUT).unwrap();

        assert_eq!(part2_internal(&points, 32).unwrap(), 16);
    }
}
