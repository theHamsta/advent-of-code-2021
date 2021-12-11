use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

const OFFSETS: [(i64, i64); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

fn offset_mut(
    array: &mut Vec<Vec<i64>>,
    (x, y): (i64, i64),
    (dx, dy): (i64, i64),
) -> Option<&mut i64> {
    array
        .get_mut((y + dy) as usize)
        .and_then(|v| v.get_mut((x + dx) as usize))
}

fn process(array: &mut Vec<Vec<i64>>, pos: (i64, i64)) -> usize {
    let mut sum = 0;
    if let Some(value) = offset_mut(array, pos, (0, 0)) {
        if *value >= 0 {
            *value += 1;
            if *value == 10 {
                sum += 1;
                sum += OFFSETS
                    .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
                    .map(|pos| process(array, pos))
                    .iter()
                    .sum::<usize>();
            }
        }
    }
    sum
}

fn step(array: &mut Vec<Vec<i64>>) -> usize {
    let mut sum = 0;
    for y in 0..array.len() {
        for x in 0..array[0].len() {
            sum += process(array, (x.try_into().unwrap(), y.try_into().unwrap()));
        }
    }

    for value in array.iter_mut().flatten() {
        if *value > 9 {
            *value = 0;
        }
    }

    sum
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let mut input: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| format!("{}", c).parse()).collect())
        .collect();

    let mut input_clone = input.clone();
    let part1: usize = (0..100).map(|_| step(&mut input_clone)).sum();
    dbg!(&part1);

    let num_octopusses = input.len() * input[0].len();
    let part2 = (0usize..)
        .map(|_| step(&mut input))
        .find_position(|&flashes| flashes == num_octopusses)
        .and_then(|(index, _)| Some(index + 1)).unwrap();
    dbg!(&part2);
    Ok(())
}
