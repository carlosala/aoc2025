use super::Solver;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Problem;

#[derive(Debug)]
pub enum Operation {
    Sum,
    Multiply,
}

#[derive(Debug)]
pub enum OpLine {
    Separator,
    Sum,
    Multiply,
}

#[derive(Debug)]
pub enum Element {
    Separator,
    Number(u8),
}

impl Solver for Problem {
    type Input = (
        (Vec<Vec<u64>>, Vec<Operation>),
        (Vec<Vec<Element>>, Vec<OpLine>),
    );
    type Output1 = u64;
    type Output2 = u64;

    fn read_input(&self, mut file_reader: BufReader<&File>) -> Self::Input {
        let mut string_file = String::new();
        file_reader.read_to_string(&mut string_file).unwrap();
        let binding = string_file
            .trim_end_matches("\n")
            .split("\n")
            .collect::<Vec<&str>>();
        let (last, rest) = binding.split_last().unwrap();
        (
            (
                rest.iter()
                    .map(|x| {
                        x.split(" ")
                            .filter_map(|x| match x {
                                "" => None,
                                x => Some(x.parse().unwrap()),
                            })
                            .collect()
                    })
                    .collect(),
                last.split(" ")
                    .filter_map(|x| match x {
                        "" => None,
                        "+" => Some(Operation::Sum),
                        "*" => Some(Operation::Multiply),
                        _ => panic!(),
                    })
                    .collect(),
            ),
            (
                rest.iter()
                    .map(|x| {
                        x.split("")
                            .filter_map(|x| match x {
                                "" => None,
                                " " => Some(Element::Separator),
                                x => Some(Element::Number(x.parse().unwrap())),
                            })
                            .collect()
                    })
                    .collect(),
                last.split("")
                    .filter_map(|x| match x {
                        "" => None,
                        " " => Some(OpLine::Separator),
                        "+" => Some(OpLine::Sum),
                        "*" => Some(OpLine::Multiply),
                        _ => panic!(),
                    })
                    .collect(),
            ),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.0 .1.iter().enumerate().fold(0, |acc, (idx, op)| {
            acc + match op {
                Operation::Sum => input.0 .0.iter().fold(0, |acc, x| acc + x[idx]),
                Operation::Multiply => input.0 .0.iter().fold(1, |acc, x| acc * x[idx]),
            }
        }))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut total = 0;
        let nums = &input.1 .0;
        let operators = &input.1 .1;
        let op_len = operators.len();
        let mut curr_nums = vec![];
        let mut skip_next = false;
        operators.iter().rev().enumerate().for_each(|(idx, op)| {
            if skip_next {
                skip_next = false;
                return;
            }
            curr_nums.push(
                nums.iter()
                    .filter_map(|v| match v[op_len - idx - 1] {
                        Element::Separator => None,
                        Element::Number(x) => Some(x),
                    })
                    .fold(0u64, |acc, next| acc * 10 + (next as u64)),
            );
            match op {
                OpLine::Separator => (),
                OpLine::Sum => {
                    skip_next = true;
                    total += curr_nums.iter().sum::<u64>();
                    curr_nums = vec![];
                }
                OpLine::Multiply => {
                    skip_next = true;
                    total += curr_nums.iter().product::<u64>();
                    curr_nums = vec![];
                }
            }
        });
        Ok(total)
    }
}
