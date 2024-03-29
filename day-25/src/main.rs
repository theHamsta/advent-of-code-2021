use std::mem::swap;

use anyhow::Context;

#[derive(thiserror::Error, Debug)]
enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Input does not have a single line")]
    NoFirstLine,
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .ok_or(AocError::NoFirstLine)?
        .chars()
        .count();
    let mut src = ndarray::Array::zeros((height, width));
    let mut dst = ndarray::Array::zeros((height, width));

    input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(y, l)| {
            l.as_bytes()
                .iter()
                .filter(|&&c| c != b' ')
                .enumerate()
                .for_each(|(x, &c)| {
                    src[(y, x)] = c;
                })
        });

    let mut step = 0;
    for _ in 0.. {
        //println!("\nStep {step}:");
        //for y in 0..height {
        //for x in 0..width {
        //print!("{}", src[(y, x)] as char);
        //}
        //println!("");
        //}

        let mut change = false;
        for y in 0..height {
            for x in 0..width {
                let center = src[(y, x)];
                let left = src[(y, (x + width - 1) % width)];
                let right = src[(y, (x + width + 1) % width)];
                dst[(y, x)] = match (center, left, right) {
                    (b'.', b'>', _) => {
                        change |= true;
                        b'>'
                    }
                    (b'>', _, b'.') => {
                        change |= true;
                        b'.'
                    }
                    (c, _, _) => c,
                }
            }
        }
        swap(&mut src, &mut dst);
        for y in 0..height {
            for x in 0..width {
                let center = src[(y, x)];
                let top = src[((y + height - 1) % height, x)];
                let bottom = src[((y + height + 1) % height, x)];
                dst[(y, x)] = match (center, top, bottom) {
                    (b'.', b'v', _) => {
                        change |= true;
                        b'v'
                    }
                    (b'v', _, b'.') => {
                        change |= true;
                        b'.'
                    }
                    (c, _, _) => c,
                }
            }
        }
        swap(&mut src, &mut dst);
        step += 1;
        if !change {
            break;
        }
    }
    println!("{step}");

    Ok(())
}
