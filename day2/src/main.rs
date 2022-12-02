// This is some of the worst stuff I've written, but it works.
//
// Somehow

use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn beats() -> HashMap<Hand, Hand> {
        HashMap::from([
            (Hand::Rock, Hand::Scissors),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Paper),
        ])
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn get_hand(letter: &str) -> Hand {
    match letter {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Unknown hand letter"),
    }
}

fn hand_to_outcome(hand: &Vec<Hand>) -> Outcome {
    match hand[1] {
        Hand::Rock => Outcome::Lose,
        Hand::Paper => Outcome::Draw,
        Hand::Scissors => Outcome::Win,
    }
}

fn outcome_to_hand(outcome: Outcome, opponent: Hand) -> Hand {
    match outcome {
        Outcome::Win => *Hand::beats().iter().find(|h| *h.1 == opponent).unwrap().0,
        Outcome::Lose => Hand::beats()[&opponent],
        Outcome::Draw => opponent,
    }
}

fn result(hand: &Vec<Hand>) -> Outcome {
    if hand[0] == hand[1] {
        return Outcome::Draw;
    }

    let beats = Hand::beats();

    if beats[&hand[1]] == hand[0] {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn main() {
    let file_path = "input1.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let rounds: Vec<_> = contents
        .split("\n")
        .map(|r| r.split(" ").map(|l| get_hand(l)).collect::<Vec<_>>())
        .collect();

    part1(&rounds);
    part2(&rounds);
}

fn part1(rounds: &Vec<Vec<Hand>>) {
    let x = rounds
        .iter()
        .map(|h| (result(&h), h[1]))
        .map(|h| h.0 as u32 + h.1 as u32);

    println!("Part 1: {:?}", x.sum::<u32>());
}

fn part2(rounds: &Vec<Vec<Hand>>) {
    let x = rounds
        .iter()
        .map(|r| (r[0], hand_to_outcome(r)))
        .map(|r| (r.0, r.1, outcome_to_hand(r.1, r.0)))
        .map(|r| r.1 as u32 + r.2 as u32);

    println!("Part 2: {:?}", x.sum::<u32>());
}
