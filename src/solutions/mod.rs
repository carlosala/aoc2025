mod solver;

pub use solver::Solver;

pub fn solve(day: usize, parts: usize, test: bool) {
    let test_str = if test { "_test" } else { "" };
    let filename = format!("inputs/{:02}{}", day, test_str);
    match day {
        _ => panic!("day not implemented"),
    }
}
