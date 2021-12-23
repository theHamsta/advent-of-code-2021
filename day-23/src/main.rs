use std::collections::HashMap;

use anyhow::Context;

use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("End of input file reached while parsing")]
    EndOfInput,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum AmphipodState {
    Parked {
        actual: u64,
        target: u64,
        move_cost: u64,
    },
    Wrong {
        actual: u64,
        target: u64,
        move_cost: u64,
        stack_depth: u64,
    },
    Correct {
        stack_depth: u64,
    },
}

impl AmphipodState {
    fn correct(&self) -> bool {
        matches!(self, AmphipodState::Correct { .. })
    }
}

fn solve_hallway_cached<const N: usize>(
    amphipods: &mut [AmphipodState; N],
    hallway_tokens: &im::HashSet<u64>,
    house_tokens: &im::HashSet<(u64, u64)>,
    cache: &mut HashMap<[AmphipodState; N], Option<u64>>,
) -> Option<u64> {
    if let Some(solution) = cache.get(amphipods) {
        *solution
    } else {
        let solution = solve_hallway(amphipods, hallway_tokens, house_tokens, cache);
        cache.insert(*amphipods, solution);
        solution
    }
}

fn solve_hallway<const N: usize>(
    amphipods: &mut [AmphipodState; N],
    hallway_tokens: &im::HashSet<u64>,
    house_tokens: &im::HashSet<(u64, u64)>,
    cache: &mut HashMap<[AmphipodState; N], Option<u64>>,
) -> Option<u64> {
    unsafe {
        static mut RECORD: usize = 0usize;

        let before = RECORD;
        RECORD = RECORD.max(
            (*amphipods)
                .into_iter()
                .filter(AmphipodState::correct)
                .count(),
        );

        if RECORD != before {
            dbg!(&amphipods);
            dbg!(&hallway_tokens);
            dbg!(&house_tokens);
            dbg!(&RECORD);
        }
    }

    if amphipods.iter().all(AmphipodState::correct) {
        Some(0)
    } else {
        (*amphipods)
            .into_iter()
            .enumerate()
            .flat_map(|(idx, s)| match s {
                AmphipodState::Correct { .. } => None,
                AmphipodState::Wrong {
                    actual,
                    target,
                    move_cost,
                    stack_depth,
                } => {
                    if (1..stack_depth).all(|i| house_tokens.contains(&(actual, i))) {
                        [
                            {
                                let hallway_free = (if actual < target {
                                    actual..=target
                                } else {
                                    target..=actual
                                })
                                .all(|i| hallway_tokens.contains(&i));
                                if hallway_free {
                                    let free_house = (1..=(N as u64 / 4))
                                        .map(|i| (target, i))
                                        .take_while(|k| house_tokens.contains(k))
                                        .last();
                                    free_house.and_then(|h| {
                                        amphipods[idx] =
                                            AmphipodState::Correct { stack_depth: h.1 };
                                        let rtn = solve_hallway_cached(
                                            amphipods,
                                            hallway_tokens,
                                            &house_tokens.update((actual, stack_depth)).without(&h),
                                            cache,
                                        )
                                        .and_then(|s| {
                                            Some(
                                                s + move_cost
                                                    * (h.1
                                                        + stack_depth
                                                        + (actual as i64 - target as i64).abs()
                                                            as u64),
                                            )
                                        });
                                        amphipods[idx] = s;
                                        rtn
                                    })
                                } else {
                                    None
                                }
                            },
                            {
                                hallway_tokens
                                    .iter()
                                    .flat_map(|&place| {
                                        let hallway_free = (if actual < place {
                                            actual..=place
                                        } else {
                                            place..=actual
                                        })
                                        .all(|i| hallway_tokens.contains(&i));
                                        if hallway_free {
                                            amphipods[idx] = AmphipodState::Parked {
                                                actual: place,
                                                target,
                                                move_cost,
                                            };
                                            let rtn = solve_hallway_cached(
                                                amphipods,
                                                &hallway_tokens.without(&place),
                                                &house_tokens.update((actual, stack_depth)),
                                                cache,
                                            )
                                            .and_then(|s| {
                                                Some(
                                                    s + move_cost
                                                        * (stack_depth
                                                            + (actual as i64 - place as i64).abs()
                                                                as u64),
                                                )
                                            });
                                            amphipods[idx] = s;
                                            rtn
                                        } else {
                                            None
                                        }
                                    })
                                    .min()
                            },
                        ]
                        .iter()
                        .flatten()
                        .min()
                        .and_then(|&a| Some(a))
                    } else {
                        None
                    }
                }
                AmphipodState::Parked {
                    actual,
                    target,
                    move_cost,
                } => {
                    let hallway_free = (if actual < target {
                        (actual + 1)..(target + 1)
                    } else {
                        target..actual
                    })
                    .all(|i| hallway_tokens.contains(&i));
                    if hallway_free {
                        let free_house = (1..=(N as u64 / 4))
                            .map(|i| (target, i))
                            .take_while(|k| house_tokens.contains(k))
                            .last();
                        free_house.and_then(|h| {
                            amphipods[idx] = AmphipodState::Correct { stack_depth: h.1 };
                            let rtn = solve_hallway_cached(
                                amphipods,
                                &hallway_tokens.update(actual),
                                &house_tokens.without(&h),
                                cache,
                            )
                            .and_then(|acc| {
                                Some(
                                    acc + move_cost
                                        * (h.1 + (actual as i64 - target as i64).abs() as u64),
                                )
                            });
                            amphipods[idx] = s;
                            rtn
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

fn parse<const N: usize>(input: &str) -> anyhow::Result<[AmphipodState; N]> {
    let mut part2_input: String;
    let input = if N == 16 {
        let mut it = input.lines();
        part2_input = "".into();
        for _ in 0..=3 {
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
    println!("{}", input);
    let re = regex::Regex::new(r"#([ABCD])#([ABCD])#([ABCD])#([ABCD])#").unwrap();
    let stack_positions: [u64; 4] = [0, 1, 2, 3].map(|i| 3 + 2 * i);
    let mut state = re
        .captures_iter(&input)
        .enumerate()
        .map(|(idx, cap)| {
            ['A', 'B', 'C', 'D'].map(|letter| {
                let actual_letter = cap[letter_to_idx(letter) as usize + 1]
                    .chars()
                    .next()
                    .unwrap();
                AmphipodState::Wrong {
                    actual: stack_positions[letter_to_idx(letter) as usize],
                    target: stack_positions[letter_to_idx(actual_letter) as usize],
                    move_cost: 10_u64.pow(letter_to_idx(actual_letter) as u32),
                    stack_depth: idx as u64 + 1,
                }
            })
        })
        .collect_vec();
    for i in 0..4 {
        state
            .iter_mut()
            .enumerate()
            .rev()
            .take_while(|(_, a)| {
                matches!(a[i], AmphipodState::Wrong{target, actual, ..} if target == actual
                )
            })
            .for_each(|(depth, a)| {
                a[i] = AmphipodState::Correct {
                    stack_depth: depth as u64 + 1,
                }
            });
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

    //let mut initial_state = parse::<8>(&input)?;

    //let mut hallway_tokens = im::HashSet::new();
    //for n in 0.."...........".len() {
    //hallway_tokens.insert(n as u64);
    //}

    //let part1 = solve_hallway(
    //&mut initial_state,
    //&hallway_tokens,
    //&im::HashSet::new(),
    //&mut HashMap::new(),
    //);
    //dbg!(&part1);
    let mut initial_state = parse::<8>(&input)?;

    let mut hallway_tokens = im::HashSet::new();
    for n in 0.."...........".len() {
        hallway_tokens.insert(n as u64);
    }

    let part2 = solve_hallway(
        &mut initial_state,
        &hallway_tokens,
        &im::HashSet::new(),
        &mut HashMap::new(),
    );
    dbg!(&part2);

    Ok(())
}
