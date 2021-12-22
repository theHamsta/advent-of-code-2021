use anyhow::Context;
use itertools::{repeat_n, Itertools};
use once_cell::sync::Lazy;

use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("End of input file reached while parsing")]
    EndOfInput,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

static DICE_SUM_FREQUENCIES: Lazy<[u64; 7]> = Lazy::new(|| {
    let mut array = [0; 7];
    repeat_n(1..=3, 3)
        .multi_cartesian_product()
        .for_each(|vec| {
            array[vec.iter().sum::<usize>() - 3] += 1;
        });
    array
});

type PlayQuantumCache = HashMap<([u64; 2], [u64; 2], bool), [u64; 2]>;

fn play_quantum(
    scores: [u64; 2],
    positions: [u64; 2],
    p1: bool,
    max_score: u64,
    cache: &mut PlayQuantumCache, // Cache not really necessary, but brings a speed-up of 3x
) -> [u64; 2] {
    match scores {
        [a, _] if a >= max_score => [1, 0],
        [_, b] if b >= max_score => [0, 1],
        [a, b] => {
            let entry = cache.get(&(scores, positions, p1));
            if let Some(entry) = entry {
                *entry
            } else {
                let rtn = DICE_SUM_FREQUENCIES
                    .iter()
                    .enumerate()
                    .map(|(idx, freq)| {
                        let sum = idx as u64 + 3;
                        let new_pos = if p1 {
                            [((positions[0] + sum - 1) % 10) + 1, positions[1]]
                        } else {
                            [positions[0], ((positions[1] + sum - 1) % 10) + 1]
                        };
                        let [x, y] = play_quantum(
                            if p1 {
                                [a + new_pos[0], b]
                            } else {
                                [a, b + new_pos[1]]
                            },
                            new_pos,
                            !p1,
                            max_score,
                            cache,
                        );
                        [freq * x, freq * y]
                    })
                    .reduce(|[a1, a2], [b1, b2]| [a1 + b1, a2 + b2])
                    .unwrap();
                cache.insert((scores, positions, p1), rtn);
                rtn
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let re = regex::Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();

    let players: Vec<(u64, u64)> = re
        .captures_iter(&input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect();

    let max_score = 1000;

    let mut scores = players.iter().map(|_| 0u64).collect_vec();
    let mut rolls: u64 = 0;
    for n in 0.. {
        scores[0] +=
            ((3 * 6 / 2 * n) * (n + 1) + (1 + 2 + 3) * (n + 1) + players[0].1 - 1) % 10 + 1;
        rolls += 3;
        if scores[0] >= max_score {
            break;
        };
        scores[1] +=
            ((3 * 6 / 2 * n) * (n + 1) + (4 + 5 + 6) * (n + 1) + players[1].1 - 1) % 10 + 1;
        rolls += 3;
        if scores[1] >= max_score {
            break;
        };
    }

    let part1: u64 = scores.iter().min().unwrap() * rolls;
    dbg!(part1);

    let mut cache = HashMap::new();
    let part2 = play_quantum([0, 0], [players[0].1, players[1].1], true, 21, &mut cache)
        .into_iter()
        .max()
        .unwrap();
    dbg!(part2);

    Ok(())
}
