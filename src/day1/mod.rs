use crate::util;

use util::Runnable;

pub struct Day1 {}

impl Runnable for Day1 {
    fn part_one(&self) {
        let max = include_str!("input.txt")
            .split("\n\n")
            .map(|x| x.split('\n').flat_map(str::parse::<usize>).sum::<usize>())
            .max();

        println!("DAY 1 \n - Part 1: {:?}", max.unwrap());
    }

    fn part_two(&self) {
        let mut max = include_str!("sample.txt")
            .split("\n\n")
            .map(|x| x.lines().flat_map(str::parse::<usize>).sum::<usize>())
            .collect::<Vec<usize>>();

        max.sort();
        max.reverse();
        let newmax = max.iter().take(3).sum::<usize>();

        println!(" - Part 2: {:?}", newmax);
        println!("=======================");
    }
}
