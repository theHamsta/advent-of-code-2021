#![feature(bool_to_option)]

use std::collections::HashMap;

use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn decode(word: &str, a: &Vec<char>, b: &Vec<char>) -> String {
    word.chars()
        .map(|c| b.get(a.iter().position(|x| c == *x).unwrap()).unwrap())
        .copied()
        .sorted()
        .collect()
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let input: Vec<Vec<Vec<&str>>> = input
        .lines()
        .map(|l| {
            l.split('|')
                .map(|l| l.split(' ').filter(|c| !c.is_empty()).collect())
                .collect()
        })
        .collect();

    let part1: usize = input
        .iter()
        .map(|l| {
            l[1].iter()
                .filter(|c| match c.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    dbg!(&part1);

    let decode_table: HashMap<&str, usize> = [
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]
    .iter()
    .copied()
    .collect();

    let char_vec: Vec<char> = "abcdefg".chars().collect();

    let part2: usize = input
        .iter()
        .flat_map(|l| {
            "abcdefg"
                .chars()
                .permutations(7)
                .flat_map(|p| {
                    for word in l[0].iter() {
                        decode_table.get(decode(word, &p, &char_vec).as_str())?;
                    }

                    let mut number = 0;
                    for word in l[1].iter() {
                        number *= 10;
                        let digit = decode_table.get(decode(word, &p, &char_vec).as_str())?;
                        number += digit;
                    }
                    Some(number)
                })
                .next()
        })
        .sum();

    dbg!(&part2);

    Ok(())
}
