use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<u8>>;
    type Output1 = u16;
    type Output2 = u64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split("").filter_map(|s| s.parse().ok()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut counter = 0;
        input.iter().for_each(|batt| {
            let mut first_num = 0u8;
            let mut second_num = 0u8;
            let batt_len = batt.len();
            for (idx, x) in batt.iter().enumerate() {
                if idx != batt_len - 1 && *x > first_num {
                    first_num = *x;
                    second_num = 0;
                } else if *x > second_num {
                    second_num = *x;
                }
            }
            counter += (first_num as u16) * 10 + (second_num as u16);
        });
        Ok(counter)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut counter = 0;
        input.iter().for_each(|batt| {
            let mut nums = [0u8; 12];
            let nums_len = nums.len();
            let batt_len = batt.len();
            for (idx, x) in batt.iter().enumerate() {
                for (num_idx, num) in nums.iter().enumerate() {
                    if batt_len - idx >= nums_len - num_idx && *x > *num {
                        nums[num_idx] = *x;
                        nums.iter_mut().skip(num_idx + 1).for_each(|x| *x = 0);
                        break;
                    }
                }
            }
            for (idx, num) in nums.iter().enumerate() {
                counter += 10u64.pow((nums_len - idx - 1) as u32) * (*num as u64)
            }
        });
        Ok(counter)
    }
}
