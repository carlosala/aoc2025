use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug, Eq, PartialEq)]
pub struct Light {
    val: u16,
}

impl Light {
    pub fn new(val: u16) -> Self {
        Self { val }
    }

    fn apply(&mut self, wire: u16) {
        self.val ^= wire;
    }
}

#[derive(Debug, Eq)]
pub struct Joltage {
    vec: Vec<u8>,
}

impl PartialEq for Joltage {
    fn eq(&self, other: &Self) -> bool {
        if self.vec.len() != other.vec.len() {
            return false;
        }
        for i in 0..self.vec.len() {
            if self.vec[i] != other.vec[i] {
                return false;
            }
        }
        true
    }
}

impl Joltage {
    pub fn new(vec: Vec<u8>) -> Self {
        Self { vec }
    }

    fn apply(&mut self, wire: u16) {
        let mut counter = 0;
        let mut curr = wire;
        while curr != 0 {
            if curr & 1 == 1 {
                self.vec[counter] += 1;
            }
            counter += 1;
            curr >>= 1;
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}

fn permuts(
    permut_len: usize,
    max_val: usize,
    opt_min_val: Option<usize>,
    part1: bool,
) -> Vec<Vec<usize>> {
    let min_val = opt_min_val.unwrap_or(0);
    if permut_len == 1 {
        return (min_val..=max_val).map(|x| vec![x]).collect();
    }
    (min_val..=max_val)
        .flat_map(|left| {
            let mut vec = permuts(
                permut_len - 1,
                max_val,
                Some(min_val + if part1 { 1 } else { 0 }),
                part1,
            );
            vec.iter_mut().for_each(|v| v.push(left));
            vec
        })
        .collect()
}

impl Solver for Problem {
    type Input = Vec<(Light, Vec<u16>, Joltage)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let vec = line.split(" ").collect::<Vec<&str>>();
                let lights = vec.first().unwrap();
                let wiring = &vec[1..(vec.len() - 1)];
                let joltage = vec.last().unwrap();
                (
                    Light::new(
                        lights
                            .chars()
                            .enumerate()
                            .fold(0u16, |acc, (idx, c)| match c {
                                '[' | ']' | '.' => acc,
                                '#' => acc + (1 << (idx - 1)),
                                _ => panic!(),
                            }),
                    ),
                    wiring
                        .iter()
                        .map(|wire| {
                            wire.get(1..(wire.len() - 1))
                                .unwrap()
                                .split(",")
                                .map(|x| x.parse::<u8>().unwrap())
                                .fold(0u16, |acc, c| acc + (1 << c))
                        })
                        .collect(),
                    Joltage::new(
                        joltage
                            .get(1..(joltage.len() - 1))
                            .unwrap()
                            .split(",")
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<u8>>(),
                    ),
                )
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().fold(0, |acc, (light, wires, _)| {
            if *light == Light::new(0) {
                return acc;
            }
            let mut len = 1;
            loop {
                for perm in permuts(len, wires.len() - 1, None, true) {
                    let mut curr = Light::new(0);
                    perm.iter().for_each(|el| curr.apply(wires[*el]));
                    if *light == curr {
                        return acc + len;
                    }
                }
                len += 1;
            }
        }))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input.iter().fold(0, |acc, (_, wires, joltage)| {
            let mut len = 1;
            loop {
                for perm in permuts(len, wires.len() - 1, None, false) {
                    let mut curr = Joltage::new(vec![0; joltage.len()]);
                    println!("{perm:?}");
                    perm.iter().for_each(|el| {
                        curr.apply(wires[*el]);
                    });
                    println!("{curr:?}");
                    if *joltage == curr {
                        return acc + len;
                    }
                }
                len += 1;
            }
        }))
    }
}
