use super::Solver;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}
impl Coordinate {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, point: &Coordinate) -> usize {
        self.x.abs_diff(point.x).pow(2)
            + self.y.abs_diff(point.y).pow(2)
            + self.z.abs_diff(point.z).pow(2)
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
                let v: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                Coordinate::new(v[0], v[1], v[2])
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut circuits: Vec<Vec<usize>> = (0..input.len()).map(|x| vec![x]).collect();
        let mut in_which_circuit: Vec<usize> = (0..input.len()).collect();
        let mut pairs: Vec<(usize, usize)> = (0..input.len())
            .flat_map(|start| ((start + 1)..input.len()).map(move |end| (start, end)))
            .collect();
        pairs.sort_by(|a, b| {
            input[a.0]
                .dist(&input[a.1])
                .cmp(&input[b.0].dist(&input[b.1]))
        });
        pairs.iter().take(1000).for_each(|p| {
            let a_circuit_idx = in_which_circuit[p.0];
            let b_circuit_idx = in_which_circuit[p.1];
            if a_circuit_idx == b_circuit_idx {
                return;
            }
            let to_add = circuits[b_circuit_idx].clone();
            circuits[a_circuit_idx].extend(to_add);
            circuits[b_circuit_idx].iter().for_each(|x| {
                in_which_circuit[*x] = a_circuit_idx;
            });
            circuits[b_circuit_idx] = vec![];
        });

        circuits.sort_by_key(|b| Reverse(b.len()));
        Ok(circuits.iter().take(3).fold(1, |acc, c| acc * c.len()))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut circuits: Vec<Vec<usize>> = (0..input.len()).map(|x| vec![x]).collect();
        let mut in_which_circuit: Vec<usize> = (0..input.len()).collect();
        let mut pairs: Vec<(usize, usize)> = (0..input.len())
            .flat_map(|start| ((start + 1)..input.len()).map(move |end| (start, end)))
            .collect();
        pairs.sort_by(|a, b| {
            input[a.0]
                .dist(&input[a.1])
                .cmp(&input[b.0].dist(&input[b.1]))
        });
        for p in pairs {
            let a_circuit_idx = in_which_circuit[p.0];
            let b_circuit_idx = in_which_circuit[p.1];
            if a_circuit_idx == b_circuit_idx {
                continue;
            }
            let to_add = circuits[b_circuit_idx].clone();
            circuits[a_circuit_idx].extend(to_add);
            if circuits[a_circuit_idx].len() == input.len() {
                return Ok(input[p.0].x * input[p.1].x);
            }
            circuits[b_circuit_idx].iter().for_each(|x| {
                in_which_circuit[*x] = a_circuit_idx;
            });
            circuits[b_circuit_idx] = vec![];
        }
        Err("UNEXPECTED".into())
    }
}
