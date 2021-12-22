use anyhow::Context;

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("End of input file reached while parsing")]
    EndOfInput,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let re =
        regex::Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
            .unwrap();

    let steps = re
        .captures_iter(&input)
        .flat_map(|cap| {
            Some((
                cap[1].to_string() == "on",
                (cap[2].parse::<i64>().ok()?, cap[3].parse::<i64>().ok()? + 1),
                (cap[4].parse::<i64>().ok()?, cap[5].parse::<i64>().ok()? + 1),
                (cap[6].parse::<i64>().ok()?, cap[7].parse::<i64>().ok()? + 1),
            ))
        })
        .collect_vec();

    let x_values = steps
        .iter()
        .flat_map(|(_, (x1, x2), _, _)| [*x1, *x2])
        .chain([-50, 50])
        .unique()
        .sorted()
        .collect_vec();

    let y_values = steps
        .iter()
        .flat_map(|(_, _, (a, b), _)| [*a, *b])
        .chain([-50, 50])
        .unique()
        .sorted()
        .collect_vec();

    let z_values = steps
        .iter()
        .flat_map(|(_, _, _, (a, b))| [*a, *b])
        .chain([-50, 50])
        .unique()
        .sorted()
        .collect_vec();

    let mut array = Array::zeros((z_values.len() - 1, y_values.len() - 1, x_values.len() - 1).f());

    for (is_on, xr, yr, zr) in steps {
        let idx0 = [zr.0, zr.1].map(|z| z_values.binary_search(&z).unwrap());
        let idx1 = [yr.0, yr.1].map(|y| y_values.binary_search(&y).unwrap());
        let idx2 = [xr.0, xr.1].map(|x| x_values.binary_search(&x).unwrap());
        array
            .slice_mut(s![idx0[0]..idx0[1], idx1[0]..idx1[1], idx2[0]..idx2[1]])
            .fill(is_on as u8);
    }

    let (part1, part2) = (0..array.shape()[0])
        .into_par_iter()
        .progress()
        .map(|z| {
            let dz = z_values[z + 1] - z_values[z];
            let mut part1 = 0;
            let mut part2 = 0;
            for y in 0..array.shape()[1] {
                let dzy = dz * (y_values[y + 1] - y_values[y]);
                for x in 0..array.shape()[2] {
                    if array[(z, y, x)] != 0 {
                        let volume = dzy * (x_values[x + 1] - x_values[x]);
                        part2 += volume as u64;
                        if -50 <= z_values[z]
                            && z_values[z] < 50
                            && -50 <= y_values[y]
                            && y_values[y] < 50
                            && -50 <= x_values[x]
                            && x_values[x] < 50
                        {
                            part1 += volume as u64;
                        }
                    }
                }
            }
            (part1, part2)
        })
        .reduce(|| (0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));

    dbg!(&part1);
    dbg!(&part2);

    Ok(())
}
