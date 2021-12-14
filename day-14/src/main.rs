#![feature(array_zip)]

use std::{collections::HashMap, rc::Rc};

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum CacheKey {
    Unexpandable((char, char)),
    Expandable((char, char), usize),
}

fn expand_chars<'cache>(
    input: (char, char),
    rules: &HashMap<(char, char), char>,
    steps: usize,
    cache: &mut HashMap<CacheKey, Rc<[u64; 26]>>,
) -> Rc<[u64; 26]> {
    let rule = rules.get(&input);
    let key;
    match rule {
        // expandable
        Some(&insertion) if steps > 0 => {
            key = CacheKey::Expandable(input, steps);
            if let Some(result) = cache.get(&key) {
                return result.clone();
            }
            let part1 = Rc::clone(&expand_chars((input.0, insertion), rules, steps - 1, cache));
            let part2 =
                Rc::clone(&expand_chars((insertion, input.1), rules, steps - 1, cache).clone());

            return Rc::clone(
                cache
                    .entry(key)
                    .or_insert(Rc::new(part1.zip(*part2).map(|(p1, p2)| p1 + p2))),
            );
        }
        // in-expandable
        _ => {
            key = CacheKey::Unexpandable(input);
            Rc::clone(cache.entry(key).or_insert_with(|| {
                let mut array = [0u64; 26];
                array[input.1 as usize - 'A' as usize] = 1;
                Rc::new(array)
            }))
        }
    }
}

fn expand(
    input: &str,
    rules: &HashMap<(char, char), char>,
    steps: usize,
    cache: &mut HashMap<CacheKey, Rc<[u64; 26]>>,
) -> Option<[u64; 26]> {
    let mut result = [0; 26];
    // Count first letter (will never be considered as second part of an unexpandable)
    result[input.chars().next()? as usize - 'A' as usize] = 1;
    for (a, b) in input.chars().tuple_windows() {
        result = expand_chars((a, b), &rules, steps, cache)
            .zip(result)
            .map(|(p1, p2)| p1 + p2);
    }
    Some(result)
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;
    let template = input
        .lines()
        .next()
        .ok_or_else(|| AocError::ParseError("No first line".to_string()))?
        .to_string();

    let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();

    let rules: HashMap<_, _> = re
        .captures_iter(&input)
        .map(|cap| {
            (
                (
                    cap[1].chars().next().unwrap(),
                    cap[2].chars().next().unwrap(),
                ),
                cap[3].chars().next().unwrap(),
            )
        })
        .collect();

    measure_time::print_time!("{:?}", "measuring block");
    //for _ in 0..1000 {
    let mut cache = HashMap::new();

    for (part, &steps) in [10, 40].iter().enumerate() {
        if let Some((min, max)) = expand(&template, &rules, steps, &mut cache)
            .unwrap()
            .iter()
            .filter(|&&f| f != 0)
            .minmax()
            .into_option()
        {
            let solution = max - min;
            println!("part {part}: {solution}");
        }
        //}
    }

    Ok(())
}
