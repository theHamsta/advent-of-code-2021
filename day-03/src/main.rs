#![feature(bool_to_option)]

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file)?;

    let mut epsilon_array = Vec::new();

    let mut numbers = Vec::new();
    for l in input.lines().filter(|l| !l.is_empty()) {
        let mut number = 0i64;
        for (pos, c) in l.chars().enumerate() {
            epsilon_array.resize(l.chars().count(), 0);
            number <<= 1;
            match c {
                '0' => {
                    epsilon_array[pos] -= 1;
                }
                '1' => {
                    epsilon_array[pos] += 1;
                    number |= 1;
                }
                _ => (),
            }
        }
        numbers.push(number);
    }

    let mut eps = 0u64;
    let mut gamma = 0u64;
    for &e in epsilon_array.iter() {
        gamma <<= 1;
        eps <<= 1;
        eps |= if e > 0 { 1 } else { 0 };
        gamma |= if e < 0 { 1 } else { 0 };
    }
    let part1 = gamma * eps;
    dbg!(&part1);

    let mut oxigen_numbers = numbers.clone();
    let num_bits = epsilon_array.len();
    for (pos, _) in epsilon_array.iter().enumerate() {
        let vote = oxigen_numbers.iter().fold(0, |acc, n| {
            if (n >> (num_bits - pos - 1)) & 1 == 1 {
                acc + 1
            } else {
                acc - 1
            }
        });
        oxigen_numbers.retain(|&number| {
            let bit_set = (number >> (num_bits - pos - 1)) & 1 == 1;
            (bit_set && vote >= 0) || (!bit_set && vote < 0)
        });
        if oxigen_numbers.len() == 1 {
            break;
        };
    }
    let oxygen = oxigen_numbers.get(0);

    let mut scuba_numbers = numbers;
    let num_bits = epsilon_array.len();
    for (pos, _) in epsilon_array.iter().enumerate() {
        let vote = scuba_numbers.iter().fold(0, |acc, n| {
            if (n >> (num_bits - pos - 1)) & 1 == 1 {
                acc + 1
            } else {
                acc - 1
            }
        });
        scuba_numbers.retain(|&number| {
            let bit_set = number >> (num_bits - pos - 1) & 1 == 1;
            (bit_set && vote < 0) || (!bit_set && vote >= 0)
        });
        if scuba_numbers.len() == 1 {
            break;
        };
    }
    let scuba = scuba_numbers.get(0);
    let part2 = scuba.unwrap() * oxygen.unwrap();
    dbg!(&part2);

    Ok(())
}
