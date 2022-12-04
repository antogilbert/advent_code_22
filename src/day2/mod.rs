use crate::util;

use util::Runnable;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const WIN: usize = 6;
const DRAW: usize = 3;

enum Move {
    Rock,
    Paper,
    Scissors,
}
struct Game {
    opponent: Move,
    me: Move,
}
pub struct Day2 {}

fn score(game: &Game) -> usize {
    match (&game.opponent, &game.me) {
        (Move::Rock, Move::Rock) => ROCK + DRAW,
        (Move::Rock, Move::Paper) => PAPER + WIN,
        (Move::Rock, Move::Scissors) => SCISSORS,
        (Move::Paper, Move::Rock) => ROCK,
        (Move::Paper, Move::Paper) => PAPER + DRAW,
        (Move::Paper, Move::Scissors) => SCISSORS + WIN,
        (Move::Scissors, Move::Rock) => ROCK + WIN,
        (Move::Scissors, Move::Paper) => PAPER,
        (Move::Scissors, Move::Scissors) => SCISSORS + DRAW,
    }
}

impl Runnable for Day2 {
    fn part_one(&self) {
        let moves = include_str!("input.txt")
            .lines()
            .map(|l| {
                let mut line = l.split_whitespace();
                Game {
                    opponent: match line.next().unwrap() {
                        "A" => Move::Rock,
                        "B" => Move::Paper,
                        "C" => Move::Scissors,
                        _ => panic!("Unknown opponent move"),
                    },
                    me: match line.next().unwrap() {
                        "X" => Move::Rock,
                        "Y" => Move::Paper,
                        "Z" => Move::Scissors,
                        _ => panic!("Unknown move"),
                    },
                }
            })
            .collect::<Vec<Game>>();

        let total = moves.iter().map(score).sum::<usize>();
        println!("DAY 2");
        println!(" - Part 1: {:?}", total);
    }

    fn part_two(&self) {
        let moves = include_str!("input.txt")
            .lines()
            .map(|l| {
                let mut line = l.split_whitespace();
                let opponent = match line.next().unwrap() {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => panic!("Unknown opponent move"),
                };
                let me = match line.next().unwrap() {
                    "X" => match opponent {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    },
                    "Y" => match opponent {
                        Move::Rock => Move::Rock,
                        Move::Paper => Move::Paper,
                        Move::Scissors => Move::Scissors,
                    },
                    "Z" => match opponent {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    },
                    _ => panic!("Unknown move"),
                };

                Game { opponent, me }
            })
            .collect::<Vec<Game>>();

        let total = moves.iter().map(score).sum::<usize>();
        println!(" - Part 2: {:?}", total);
    }
}
