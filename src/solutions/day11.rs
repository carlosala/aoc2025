use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(String, Vec<String>)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| -> (String, Vec<String>) {
                let vec = line.split(": ").map(Into::into).collect::<Vec<String>>();
                (vec[0].clone(), vec[1].split(" ").map(Into::into).collect())
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut map = HashMap::<&str, Vec<&str>>::new();
        input.iter().for_each(|(from, to)| {
            map.insert(from.as_str(), to.iter().map(|x| x.as_str()).collect());
        });
        fn paths_to_out(map: &HashMap<&str, Vec<&str>>, to_visit: &str) -> usize {
            let node = map.get(to_visit).unwrap();
            let mut total = 0;
            for v in node {
                match v {
                    a if *a == "out" => total += 1,
                    a => total += paths_to_out(map, a),
                };
            }
            total
        }
        Ok(paths_to_out(&map, "you"))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut map = HashMap::<&str, Vec<&str>>::new();
        input.iter().for_each(|(from, to)| {
            map.insert(from.as_str(), to.iter().map(|x| x.as_str()).collect());
        });
        fn paths_to<'a>(
            map: &HashMap<&'a str, Vec<&'a str>>,
            cache: &mut HashMap<&'a str, usize>,
            to_visit: &'a str,
            target: &'a str,
        ) -> usize {
            if let Some(cached) = cache.get(to_visit) {
                return *cached;
            }
            let node = map.get(to_visit).unwrap();
            let mut total = 0;
            for v in node {
                match v {
                    a if *a == target => {
                        total += 1;
                    }
                    a if *a == "out" => (),
                    _ => {
                        total += paths_to(map, cache, v, target);
                    }
                };
            }
            cache.insert(to_visit, total);
            total
        }
        let total = paths_to(&map, &mut HashMap::new(), "svr", "dac")
            * paths_to(&map, &mut HashMap::new(), "dac", "fft")
            * paths_to(&map, &mut HashMap::new(), "fft", "out")
            + paths_to(&map, &mut HashMap::new(), "svr", "fft")
                * paths_to(&map, &mut HashMap::new(), "fft", "dac")
                * paths_to(&map, &mut HashMap::new(), "dac", "out");

        Ok(total)
    }
}
