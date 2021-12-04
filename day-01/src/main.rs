use itertools::*;

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file)?;

    let part1 = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|n| n.parse::<i64>())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    dbg!(&part1);

    let part2_alt = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|n| n.parse::<i64>())
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count();
    dbg!(&part2_alt);
    Ok(())
}
