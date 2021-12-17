use std::ops::RangeInclusive;

use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Debug, Default)]
struct ProbeState {
    pos: (i64, i64),
    vel: (i64, i64),
}

impl ProbeState {
    fn step(&mut self) {
        let Self {
            pos: (x, y),
            vel: (vx, vy),
        } = self;
        self.pos = (*x + *vx, *y + *vy);
        self.vel = (*vx - vx.signum(), *vy - 1);
    }
}

#[derive(Debug)]
struct TargetArea {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

impl TargetArea {
    fn contains(&self, (x, y): (i64, i64)) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;
    let re = regex::Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();

    let cap = re
        .captures_iter(&input)
        .next()
        .ok_or_else(|| AocError::ParseError("regex didn't match".to_string()))?;

    let x1: i64 = cap[1].parse()?;
    let x2: i64 = cap[2].parse()?;
    let y1: i64 = cap[3].parse()?;
    let y2: i64 = cap[4].parse()?;

    let target_area = TargetArea {
        x_range: x1.min(x2)..=x1.max(x2),
        y_range: y1.min(y2)..=y1.max(y2),
    };

    let mut part2 = 0;

    let part1 = (0..400i64)
        // .filter(|n| target_area.x_range.contains(&(n * (n + 1) / 2))) // 0 x-vel at end
        .cartesian_product(-400..400)
        .flat_map(|vel| {
            let mut state = ProbeState::default();
            state.vel = vel;
            let mut height = i64::MIN;
            while &state.pos.1 >= target_area.y_range.start() {
                state.step();
                height = height.max(state.pos.1);
                if target_area.contains(state.pos) {
                    part2 += 1;
                    return Some(height);
                }
            }
            None
        })
        .max();
    dbg!(part1);
    dbg!(part2);

    Ok(())
}
