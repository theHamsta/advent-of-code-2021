use std::collections::{HashMap, HashSet};

use anyhow::Context;

use itertools::Itertools;
use once_cell::unsync::Lazy;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("End of input file reached while parsing")]
    EndOfInput,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum State {
    Parked {
        actual: u8,
        target: u8,
        move_cost: u16,
    },
    Wrong {
        actual: u8,
        target: u8,
        move_cost: u16,
        stack_depth: u8,
    },
    Correct,
}

impl State {
    fn correct(&self) -> bool {
        matches!(self, State::Correct)
    }

    fn target(&self) -> Option<u8> {
        match self {
            Self::Correct => None,
            &Self::Wrong { target, .. } | &Self::Parked { target, .. } => Some(target),
        }
    }

    fn parking_pos(&self) -> Option<u8> {
        match self {
            &Self::Parked { actual, .. } => Some(actual),
            _ => None,
        }
    }
}

fn solve_hallway_cached<const N: usize>(
    amphipods: &mut [State; N],
    hallway_tokens: &mut [bool; 11],
    house_tokens: &mut HashSet<(u8, u8)>,
    cache: &mut HashMap<[State; N], Option<u64>>,
) -> Option<u64> {
    let mut key = amphipods.clone();
    key.sort();
    if let Some(solution) = cache.get(&key) {
        solution.clone()
    } else {
        let solution = solve_hallway(amphipods, hallway_tokens, house_tokens, cache);
        cache.insert(key, solution.clone());
        solution
    }
}

fn solve_hallway<const N: usize>(
    state: &mut [State; N],
    hallway_tokens: &mut [bool; 11],
    house_tokens: &mut HashSet<(u8, u8)>,
    cache: &mut HashMap<[State; N], Option<u64>>,
) -> Option<u64> {
    assert!(
        state.into_iter().flat_map(|s| s.parking_pos()).count()
            == state
                .into_iter()
                .flat_map(|s| s.parking_pos())
                .unique()
                .count()
    );

    if state.iter().all(State::correct) {
        Some(0 /*, "Finished".into()*/)
    } else {
        (*state)
            .into_iter()
            .enumerate()
            .flat_map(|(idx, s)| match s {
                State::Correct => None,
                State::Wrong {
                    actual,
                    target,
                    move_cost,
                    stack_depth,
                } => {
                    if (1..stack_depth).all(|i| house_tokens.contains(&(actual, i))) {
                        {
                            (0..hallway_tokens.len())
                                .flat_map(|place| {
                                    let actual = actual as usize;
                                    let hallway_free = (if actual < place {
                                        actual..=place
                                    } else {
                                        place..=actual
                                    })
                                    .all(|i| hallway_tokens[i]);
                                    if hallway_free
                                        && place != 2 // Very stupid rule that I didn't read! They never stop before their house otherwise you can achieve a 20 points lower score in the example and it will take much longer
                                        && place != 4
                                        && place != 6
                                        && place != 8
                                    {
                                        state[idx] = State::Parked {
                                            actual: place as u8,
                                            target,
                                            move_cost,
                                        };
                                        hallway_tokens[place] = false;
                                        house_tokens.insert((actual as u8, stack_depth));
                                        let rtn = solve_hallway_cached(
                                            state,
                                            hallway_tokens,
                                            house_tokens,
                                            cache,
                                        )
                                        .and_then(|s| {
                                            let cost = move_cost as u64
                                                * (stack_depth as u64
                                                    + (actual as i64 - place as i64).abs() as u64);
                                            Some(
                                                s + cost,
                                                //format!(
                                                //"parked {} from {} at {} (cost {cost})\n{}",
                                                //target_to_letter(target),
                                                //actual,
                                                //place,
                                                //s.1
                                                //),
                                            )
                                        });
                                        house_tokens.remove(&(actual as u8, stack_depth));
                                        hallway_tokens[place] = true;
                                        state[idx] = s;
                                        rtn
                                    } else {
                                        None
                                    }
                                })
                                .min()
                        }
                    } else {
                        None
                    }
                }
                State::Parked {
                    actual,
                    target,
                    move_cost,
                } => {
                    let hallway_free = (if actual < target {
                        (actual + 1)..(target + 1)
                    } else {
                        target..actual
                    })
                    .all(|i| hallway_tokens[i as usize]);
                    if hallway_free {
                        let free_house = (1..=(N as u8 / 4))
                            .map(|i| (target, i))
                            .take_while(|k| house_tokens.contains(k))
                            .last();
                        free_house.and_then(|h| {
                            if h.1 as usize
                                == state
                                    .into_iter()
                                    .filter(|s| s.target() == Some(target))
                                    .count()
                            {
                                state[idx] = State::Correct;
                                hallway_tokens[actual as usize] = true;
                                house_tokens.remove(&h);
                                let rtn = solve_hallway_cached(
                                    state,
                                    hallway_tokens,
                                    house_tokens,
                                    cache,
                                )
                                .and_then(|acc| {
                                    let cost = move_cost as u64
                                        * (h.1 as u64
                                            + (actual as i64 - target as i64).abs() as u64);
                                    Some(
                                        acc + cost,
                                        //format!(
                                        //"home {} from {} depth {} (cost {cost})\n{}",
                                        //target_to_letter(target),
                                        //actual,
                                        //h.1,
                                        //acc.1
                                        //),
                                    )
                                });
                                house_tokens.insert(h);
                                hallway_tokens[actual as usize] = false;
                                state[idx] = s;
                                rtn
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                }
            })
            .min()
    }
}

fn letter_to_idx(c: char) -> u64 {
    (c as u64) - ('A' as u64)
}
//fn target_to_letter(c: u8) -> char {
//(((c - 2) / 2) + ('A' as u8)) as char
//}

fn parse<const N: usize>(input: &str) -> anyhow::Result<[State; N]> {
    let mut part2_input: String;
    let input = if N == 16 {
        let mut it = input.lines();
        part2_input = "".into();
        for _ in 0..=2 {
            part2_input.push_str(it.next().ok_or(AocError::EndOfInput)?);
            part2_input.push('\n');
        }
        part2_input.push_str("#D#C#B#A#\n");
        part2_input.push_str("#D#B#A#C#\n");
        part2_input.push_str(it.next().ok_or(AocError::EndOfInput)?);
        part2_input.push('\n');
        &part2_input
    } else {
        input
    };

    let re = Lazy::new(|| regex::Regex::new(r"#([ABCD])#([ABCD])#([ABCD])#([ABCD])#").unwrap());
    let stack_positions: [u8; 4] = [0, 1, 2, 3].map(|i| 2 + 2 * i);
    let mut state = re
        .captures_iter(&input)
        .enumerate()
        .map(|(idx, cap)| {
            ['A', 'B', 'C', 'D'].map(|letter| {
                let actual_letter = cap[letter_to_idx(letter) as usize + 1]
                    .chars()
                    .next()
                    .unwrap();
                State::Wrong {
                    actual: stack_positions[letter_to_idx(letter) as usize],
                    target: stack_positions[letter_to_idx(actual_letter) as usize],
                    move_cost: 10_u64.pow(letter_to_idx(actual_letter) as u32) as u16,
                    stack_depth: idx as u8 + 1,
                }
            })
        })
        .collect_vec();
    for i in 0..4 {
        state
            .iter_mut()
            .rev()
            .take_while(|a| {
                matches!(a[i], State::Wrong{target, actual, ..} if target == actual
                )
            })
            .for_each(|a| a[i] = State::Correct);
    }

    Ok(state
        .into_iter()
        .flatten()
        .collect_vec()
        .try_into()
        .map_err(|_| AocError::ParseError(format!("Expected {N} amphipods")))?)
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let mut initial_state = parse::<8>(&input)?;
    let mut hallway_tokens = [true; "...........".len()];

    let part1 = solve_hallway(
        &mut initial_state,
        &mut hallway_tokens,
        &mut HashSet::new(),
        &mut HashMap::new(),
    );
    dbg!(&part1);

    let mut initial_state = parse::<16>(&input)?;
    let mut hallway_tokens = [true; "...........".len()];

    let part2 = solve_hallway(
        &mut initial_state,
        &mut hallway_tokens,
        &mut HashSet::new(),
        &mut HashMap::new(),
    );
    dbg!(&part2);

    Ok(())
}
