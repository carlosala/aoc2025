use super::Solver;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<(u64, u64)>, Vec<u64>);
    type Output1 = u16;
    type Output2 = u64;

    fn read_input(&self, mut file_reader: BufReader<&File>) -> Self::Input {
        let mut file = String::new();
        file_reader.read_to_string(&mut file).unwrap();
        let parts: Vec<&str> = file.trim().split("\n\n").collect();
        (
            parts[0]
                .split("\n")
                .map(|r| {
                    let range: Vec<u64> = r.split("-").map(|a| a.parse().unwrap()).collect();
                    (range[0], range[1])
                })
                .collect(),
            parts[1].split("\n").map(|x| x.parse().unwrap()).collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut counter = 0;
        let mut all_ranges = input.0.clone();
        all_ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let mut ranges = vec![];
        let mut curr = all_ranges[0];
        all_ranges.iter().skip(1).for_each(|range| {
            if curr.1 >= range.0 {
                curr.1 = range.1.max(curr.1);
            } else {
                ranges.push(curr);
                curr = *range;
            }
        });
        ranges.push(curr);
        let mut ids = input.1.clone();
        ids.sort();

        let mut ids_iter = ids.iter();
        let mut ranges_iter = ranges.iter();
        let mut curr_id = ids_iter.next();
        let mut curr_range = ranges_iter.next();

        while let Some((id, range)) = curr_id.zip(curr_range) {
            if *id < range.0 {
                curr_id = ids_iter.next();
                continue;
            }
            if *id <= range.1 {
                counter += 1;
                curr_id = ids_iter.next();
                continue;
            }
            curr_range = ranges_iter.next();
        }

        Ok(counter)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut counter = 0;
        let mut all_ranges = input.0.clone();
        all_ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let mut curr = all_ranges[0];
        all_ranges.iter().skip(1).for_each(|range| {
            if curr.1 >= range.0 {
                curr.1 = range.1.max(curr.1);
            } else {
                counter += curr.1 - curr.0 + 1;
                curr = *range;
            }
        });
        counter += curr.1 - curr.0 + 1;
        Ok(counter)
    }
}
