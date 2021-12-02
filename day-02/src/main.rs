#[derive(Debug)]
struct Command {
    direction: String,
    amount: i64,
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file)?;

    let mut x = 0;
    let mut depth = 0;

    let commands: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|c| {
            let mut capture = c.split(' ');
            Some(Command {
                direction: capture.next()?.into(),
                amount: capture.next()?.parse().ok()?,
            })
        })
        .collect();

    for c in commands.iter() {
        match c.direction.as_str() {
            "up" => depth -= c.amount,
            "down" => depth += c.amount,
            "backward" => x -= c.amount,
            "forward" => x += c.amount,
            _ => (),
        }
    }

    dbg!(depth * x);

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for c in commands {
        match c.direction.as_str() {
            "up" => {
                aim -= c.amount;
            }
            "down" => {
                aim += c.amount;
            }
            "forward" => {
                x += c.amount;
                depth += aim * c.amount;
            }
            _ => (),
        }
    }
    dbg!(depth * x);
    Ok(())
}
