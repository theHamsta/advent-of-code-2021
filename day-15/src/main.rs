#![feature(array_zip)]

use std::cmp::min;
use std::collections::HashSet;

use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

const OFFSETS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn offset(
    array: &Vec<Vec<i64>>,
    (x, y): (i64, i64),
    (dx, dy): (i64, i64),
    extend: bool,
) -> Option<i64> {
    if extend {
        let penalty: i64 = ((x + dx) as usize / array[0].len() + (y + dy) as usize / array.len())
            .try_into()
            .unwrap();
        array
            .get((y + dy) as usize % array.len())
            .and_then(|v| v.get((x + dx) as usize % array[0].len()))
            .and_then(|&value| Some((((value + penalty) - 1) % 9) + 1))
    } else {
        array
            .get((y + dy) as usize)
            .and_then(|v| v.get((x + dx) as usize))
            .and_then(|&v| Some(v))
    }
}
fn offset_mut<'array>(
    array: &'array mut Vec<Vec<i64>>,
    (x, y): (i64, i64),
    (dx, dy): (i64, i64),
) -> Option<&'array mut i64> {
    array
        .get_mut((y + dy) as usize)
        .and_then(|v| v.get_mut((x + dx) as usize))
}

fn get(array: &Vec<Vec<i64>>, pos: (i64, i64)) -> Option<i64> {
    offset(array, pos, (0, 0), false)
}

fn neighbors(array: &Vec<Vec<i64>>, pos: (i64, i64), extend: bool) -> [Option<i64>; 4] {
    OFFSETS.map(|o| offset(array, pos, o, extend))
}

fn dijkstra(input: &Vec<Vec<i64>>, part2: bool) -> anyhow::Result<i64> {
    let start = (0, 0);
    let mut goal = (
        (input[0].len() - 1).try_into()?,
        (input.len() - 1).try_into()?,
    );
    if part2 {
        goal = (
            (input[0].len() * 5 - 1).try_into()?,
            (input.len() * 5 - 1).try_into()?,
        )
    }

    let mut prio_queue = priority_queue::PriorityQueue::new();
    prio_queue.push(start, -0);

    let mut weights = Vec::new();
    weights.resize_with(goal.1 as usize + 1, || {
        let mut vec = Vec::new();
        vec.resize(goal.0 as usize + 1, i64::MAX);
        vec
    });

    let mut visited = HashSet::new();
    while let Some((current, current_neg_weight)) = prio_queue.pop() {
        if current == goal {
            break;
        }
        visited.insert(current);
        let nodes = neighbors(&input, current, part2).zip(OFFSETS);

        nodes
            .iter()
            .flat_map(|&(n, offset)| Some((n?, offset)))
            .for_each(|(n, offset)| {
                let pos = (current.0 + offset.0, current.1 + offset.1);
                if !visited.contains(&pos) {
                    let neightbor_weight = offset_mut(&mut weights, current, offset);
                    if let Some(neightbor_weight) = neightbor_weight {
                        *neightbor_weight = min(*neightbor_weight, n - current_neg_weight);
                        prio_queue.push(pos, -*neightbor_weight);
                    }
                }
            });
    }
    Ok(get(&weights, goal).unwrap())
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;
    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| format!("{}", c).parse::<i64>())
                .collect_vec()
        })
        .collect_vec();

    let part1 = dijkstra(&input, false)?;
    dbg!(&part1);

    let part2 = dijkstra(&input, true)?;
    dbg!(&part2);

    Ok(())
}
