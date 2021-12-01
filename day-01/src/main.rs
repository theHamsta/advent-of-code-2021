use itertools::*;

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file)?;

    let part1 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|n| n.parse::<i64>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    dbg!(&part1);

    let part2 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|n| n.parse::<i64>().unwrap())
        .tuple_windows()
        .tuple_windows()
        .filter(|((a1, b1, c1), (a2, b2, c2))| a1 + b1 + c1 < a2 + b2 + c2)
        .count();
    dbg!(&part2);

    Ok(())
}
