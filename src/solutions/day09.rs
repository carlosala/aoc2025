use super::Solver;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

pub struct Problem;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x.abs_diff(rhs.x), self.y.abs_diff(rhs.y))
    }
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn area(&self, point: &Coordinate) -> usize {
        (self.x.abs_diff(point.x) + 1) * (self.y.abs_diff(point.y) + 1)
    }
}

impl Solver for Problem {
    type Input = Vec<Coordinate>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let vec: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                Coordinate::new(vec[0], vec[1])
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok((0..input.len())
            .flat_map(|start| ((start + 1)..input.len()).map(move |end| (start, end)))
            .fold(0, |acc, p| acc.max(input[p.0].area(&input[p.1]))))
    }

    fn solve_second(&self, _input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
