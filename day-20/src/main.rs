use anyhow::Context;
use itertools::Itertools;

use std::{
    collections::HashSet,
    fmt::{Display, Write},
    mem::swap,
    ops::RangeInclusive,
};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("End of input file reached while parsing")]
    EndOfInput,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Debug)]
struct Image {
    decode_string: String,
    pixels: HashSet<(i64, i64)>,
    range: (RangeInclusive<i64>, RangeInclusive<i64>),
    background_pixel: u8,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.range.1.clone() {
            for x in self.range.0.clone() {
                if self.pixels.contains(&(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Image {
    fn new(decode_string: String) -> Self {
        Self {
            decode_string,
            pixels: HashSet::new(),
            range: (0..=0, 0..=0),
            background_pixel: b'.',
        }
    }
    fn from_str(input: &str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let decode_string = lines.next().ok_or(AocError::EndOfInput)?.into();
        let mut x_max = 0i64;
        let mut y_max = 0i64;
        let pixels = lines
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, l)| {
                y_max = y_max.max(y as i64);
                l.chars()
                    .enumerate()
                    .flat_map(|(x, c)| {
                        if c == '#' {
                            x_max = x_max.max(x as i64);
                            Some((x as i64, y as i64))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect();
        Ok(Self {
            decode_string,
            pixels,
            range: (0..=x_max, 0..=y_max),
            background_pixel: b'.',
        })
    }

    fn step(&mut self, next: &mut Image) {
        let Self {
            range: (x_range, y_range),
            pixels,
            background_pixel,
            ..
        } = self;
        next.pixels.clear();
        next.range = (
            (x_range.start() - 1)..=(x_range.end() + 1),
            (y_range.start() - 1)..=(y_range.end() + 1),
        );

        for y in next.range.1.clone() {
            for x in next.range.0.clone() {
                let mut neightborhood_encoded = 0usize;
                for (dy, dx) in (-1..=1).cartesian_product(-1..=1) {
                    neightborhood_encoded <<= 1;
                    neightborhood_encoded |= if !RangeInclusive::contains(x_range, &(x + dx))
                        || !RangeInclusive::contains(y_range, &(y + dy))
                    {
                        (*background_pixel == b'#') as usize
                    } else {
                        pixels.contains(&(x + dx, y + dy)) as usize
                    };
                }
                let pixel = self.decode_string.as_bytes()[neightborhood_encoded] == b'#';
                if pixel {
                    next.pixels.insert((x, y));
                }
            }
        }
        if *background_pixel == b'.' {
            next.background_pixel = self.decode_string.as_bytes()[0];
        } else {
            next.background_pixel = self.decode_string.as_bytes()[511];
        }
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let mut image = Image::from_str(&input)?;
    let mut next_image = Image::new(image.decode_string.clone());

    //println!("{input_image}");
    for _step in 1..=2 {
        image.step(&mut next_image);
        swap(&mut image, &mut next_image);
        //println!("Step {_step}:\n{input_image}");
    }
    let part1 = image.pixels.len();
    dbg!(part1);
    for _step in 3..=50 {
        image.step(&mut next_image);
        swap(&mut image, &mut next_image);
    }
    let part2 = image.pixels.len();
    dbg!(part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotation_has_identity() {}
}
