use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (usize, Vec<Vec<usize>>);
    type Output1 = u32;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut lines = file_reader.lines();
        let initial_pos = lines.next().unwrap().unwrap().find("S").unwrap();
        let nums = lines
            .map(|x| {
                x.unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, c)| match c {
                        '^' => Some(idx),
                        '.' => None,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        (initial_pos, nums)
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut counter = 0;
        let mut curr_beams = HashSet::new();
        curr_beams.insert(input.0);
        input.1.iter().for_each(|line| {
            let mut next_beams = curr_beams.clone();
            curr_beams.iter().for_each(|b| {
                if line.contains(b) {
                    next_beams.remove(b);
                    next_beams.insert(b - 1);
                    next_beams.insert(b + 1);
                    counter += 1;
                }
            });
            curr_beams = next_beams;
        });
        Ok(counter)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut curr_beams = HashMap::new();
        curr_beams.insert(input.0, 1);
        input.1.iter().for_each(|line| {
            let mut next_beams = curr_beams.clone();
            curr_beams.iter().for_each(|(pos, counter)| {
                if line.contains(pos) {
                    *next_beams.entry(pos - 1).or_insert(0) += counter;
                    *next_beams.entry(pos + 1).or_insert(0) += counter;
                    if let Some(x) = next_beams.get(pos) {
                        if x <= counter {
                            next_beams.remove(pos);
                        } else {
                            next_beams.entry(*pos).and_modify(|v| *v -= counter);
                        }
                    };
                }
            });
            curr_beams = next_beams;
        });
        Ok(curr_beams.iter().fold(0, |acc, n| acc + n.1))
    }
}
