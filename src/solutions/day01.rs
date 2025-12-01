use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(i32, i32)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let dir = match line.chars().next().unwrap() {
                    'L' => -1,
                    'R' => 1,
                    _ => {
                        println!("{}", line);
                        panic!("UNEXPECTED CHAR")
                    }
                };
                let num: i32 = line.get(1..).unwrap().parse().unwrap();
                (dir, num)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut count: usize = 0;
        input.iter().fold(50, |acc, &v| {
            let new_acc = (acc + v.0 * v.1) % 100;
            if new_acc == 0 {
                count += 1;
            };
            new_acc
        });
        Ok(count)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut count: usize = 0;
        input.iter().fold(50, |acc, &v| {
            count += (v.1 as usize) / 100;
            let mut new_acc = acc + v.0 * (v.1 % 100);
            if new_acc <= 0 || new_acc >= 100 {
                // remove negatives
                new_acc += 100;
                if acc != 0 {
                    count += 1;
                };
            };
            new_acc % 100
        });
        Ok(count)
    }
}
