use advent_code_22::*;
use util::Runnable;

fn main() {
    let day = std::env::args().nth(1).expect("NO DAY PROVIDED");

    let runners: Vec<Box<dyn Runnable>> = vec![Box::new(day1::Day1 {}), Box::new(day2::Day2 {})];

    match day.parse::<usize>() {
        Ok(d) => {
            runners[d - 1].part_one();
            runners[d - 1].part_two();
        }
        Err(_) => {
            runners.iter().for_each(|r| {
                r.part_one();
                r.part_two();
            });
        }
    }
}
