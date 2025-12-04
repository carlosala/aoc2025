use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|range| {
                let split: Vec<u64> = range.split("-").map(|a| a.parse().unwrap()).collect();
                (split[0], split[1])
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut count = 0;
        input.iter().for_each(|(lower, upper)| {
            let lower_digits = lower.ilog10() + 1;
            let mut curr = if lower_digits % 2 == 0 {
                lower / 10u64.pow(lower_digits / 2)
            } else {
                10u64.pow(lower_digits / 2)
            };
            loop {
                let val = curr + curr * 10u64.pow(curr.ilog10() + 1);
                if val > *upper {
                    break;
                }
                if val >= *lower {
                    count += val
                };
                curr += 1;
            }
        });
        Ok(count)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut invalid = HashSet::new();
        input.iter().for_each(|(lower, upper)| {
            let lower_digits = lower.ilog10() + 1;
            let upper_digits = upper.ilog10() + 1;
            for times in 2..(upper_digits + 1) {
                let mut curr = if lower_digits % times == 0 {
                    lower / (10u64.pow(lower_digits - lower_digits / times))
                } else {
                    10u64.pow(lower_digits / times)
                };
                loop {
                    let mut val = curr;
                    let digits = curr.ilog10() + 1;
                    for pow in 1..times {
                        val += curr * 10u64.pow(pow * digits)
                    }
                    if val > *upper {
                        break;
                    }
                    if val >= *lower {
                        invalid.insert(val);
                    };
                    curr += 1;
                }
            }
        });
        Ok(invalid.iter().sum())
    }
}
