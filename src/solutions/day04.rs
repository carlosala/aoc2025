use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn calculate_total(input: &[Vec<u8>]) -> u16 {
    input
        .iter()
        .fold(0, |acc, line| acc + line.iter().sum::<u8>() as u16)
}

const PAIRS: [(i16, i16); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, 0),
    (1, -1),
    (1, 1),
];

impl Solver for Problem {
    type Input = Vec<Vec<u8>>;
    type Output1 = u16;
    type Output2 = u16;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| {
                x.unwrap()
                    .split("")
                    .filter_map(|c| match c {
                        "." => Some(0),
                        "@" => Some(1),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut total = calculate_total(input);
        let x_len = input.len();
        let y_len = input[0].len();
        let mut counter = vec![vec![0u8; y_len]; x_len];
        for (x_idx, x) in input.iter().enumerate() {
            for (y_idx, num) in x.iter().enumerate() {
                if *num == 0 {
                    continue;
                }
                PAIRS.iter().for_each(|(x_diff, y_diff)| {
                    let x_coord = x_idx as i16 + x_diff;
                    let y_coord = y_idx as i16 + y_diff;
                    if x_coord >= 0
                        && (x_coord as usize) < x_len
                        && y_coord >= 0
                        && (y_coord as usize) < y_len
                    {
                        if input[x_coord as usize][y_coord as usize] == 0 {
                            return;
                        }
                        counter[x_coord as usize][y_coord as usize] += 1;
                        if counter[x_coord as usize][y_coord as usize] == 4 {
                            total -= 1;
                        };
                    }
                });
            }
        }
        Ok(total)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let rolls = calculate_total(input);
        let x_len = input.len();
        let y_len = input[0].len();
        let mut input = input.clone();
        loop {
            let mut next_input = vec![vec![0u8; y_len]; x_len];
            let mut counter = vec![vec![0u8; y_len]; x_len];
            for (x_idx, x) in input.iter().enumerate() {
                for (y_idx, num) in x.iter().enumerate() {
                    if *num == 0 {
                        continue;
                    }
                    PAIRS.iter().for_each(|(x_diff, y_diff)| {
                        let x_coord = x_idx as i16 + x_diff;
                        let y_coord = y_idx as i16 + y_diff;
                        if x_coord >= 0
                            && (x_coord as usize) < x_len
                            && y_coord >= 0
                            && (y_coord as usize) < y_len
                        {
                            if input[x_coord as usize][y_coord as usize] == 0 {
                                return;
                            }
                            counter[x_coord as usize][y_coord as usize] += 1;
                            if counter[x_coord as usize][y_coord as usize] == 4 {
                                next_input[x_coord as usize][y_coord as usize] = 1;
                            };
                        }
                    });
                }
            }
            let next_total = calculate_total(&next_input);
            if next_total == calculate_total(&input) {
                return Ok(rolls - next_total);
            }
            input = next_input;
        }
    }
}
