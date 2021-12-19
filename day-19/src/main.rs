use anyhow::Context;
use cgmath::{Deg, InnerSpace, Matrix3, Vector3};
use itertools::{repeat_n, Itertools};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

type Coord = Vector3<i64>;

static ROTATIONS: Lazy<Vec<Matrix3<f64>>> = Lazy::new(get_rotations);

fn parse(input: &str) -> anyhow::Result<HashMap<String, HashSet<Coord>>> {
    let mut detections = HashMap::new();
    let re_section = Lazy::new(|| regex::Regex::new(r"--- scanner (\d+) ---").unwrap());

    let mut lines = input.lines().filter(|l| !l.is_empty());
    let mut current: String = re_section
        .captures(lines.next().ok_or(AocError::ParseError(
            "input does not have a first line".into(),
        ))?)
        .ok_or(AocError::ParseError("regex didn't match".into()))?[1]
        .parse()?;

    for l in lines {
        if let Some(new_section) = re_section.captures(l) {
            current = new_section[1].into();
        } else {
            detections
                .entry(current.clone())
                .or_insert_with(|| HashSet::new())
                .insert({
                    let mut it = l.split(',');
                    Vector3::new(
                        it.next()
                            .ok_or_else(|| {
                                AocError::ParseError(format!("could not parse coord '{}'", &l))
                            })?
                            .parse()?,
                        it.next()
                            .ok_or_else(|| {
                                AocError::ParseError(format!("could not parse coord '{}'", &l))
                            })?
                            .parse()?,
                        it.next().unwrap_or("0").parse()?,
                    )
                });
        }
    }
    Ok(detections)
}

fn rotate(v: &Vector3<i64>, rot: &Matrix3<f64>) -> Vector3<i64> {
    let result = rot * Vector3::<f64>::new(v[0] as f64, v[1] as f64, v[2] as f64);
    Vector3::<i64>::new(
        result[0].round() as i64,
        result[1].round() as i64,
        result[2].round() as i64,
    )
}

fn get_rotations() -> Vec<Matrix3<f64>> {
    repeat_n(0..4, 3)
        .multi_cartesian_product()
        .map(|vec| {
            Matrix3::<f64>::from_angle_x(Deg(90. * vec[0] as f64))
                * Matrix3::<f64>::from_angle_y(Deg(90. * vec[1] as f64))
                * Matrix3::<f64>::from_angle_z(Deg(90. * vec[2] as f64))
        })
        .collect()
}

struct Scanner {
    id: String,
    scanners: Vec<Coord>,
    points: HashSet<Coord>,
    angle_features: HashMap<[i64; 3], HashSet<Coord>>,
}

impl Scanner {
    fn new<Points>(id: String, points: Points) -> Self
    where
        Points: Iterator<Item = Coord>,
    {
        let mut rtn = Self {
            id,
            scanners: vec![Vector3::new(0, 0, 0)],
            points: points.collect(),
            angle_features: HashMap::new(),
        };

        // distances instead of triangle angles would have been better!
        rtn.points.iter().permutations(3).for_each(|vec| {
            let a = vec[1] - vec[0];
            let b = vec[2] - vec[0];
            let c = vec[2] - vec[1];
            let key = [a.dot(b), b.dot(c), c.dot(a)];
            rtn.angle_features
                .entry(key)
                .or_insert_with(|| HashSet::new())
                .insert(*vec[0]);
        });

        rtn
    }

    fn align_with_other(&self, other: &Self, min_overlapping: u64) -> Option<Self> {
        let mut correspondences = HashMap::<Coord, HashSet<Coord>>::new();
        for (tri, points) in self.angle_features.iter() {
            for point in points.iter() {
                if let Some(correspondence) = other.angle_features.get(tri) {
                    correspondences
                        .entry(*point)
                        .or_insert_with(HashSet::new)
                        .extend(correspondence);
                }
            }
        }

        if correspondences.len() >= min_overlapping as usize {
            let mut shifts = HashMap::new();
            ROTATIONS.iter().enumerate().for_each(|(idx, rot)| {
                for (a, bs) in correspondences.iter() {
                    for b in bs {
                        *shifts.entry((idx, rotate(&a, &rot) - b)).or_insert(0) += 1
                    }
                }
            });
            let ((idx, shift), _) = shifts.iter().max_by_key(|(_, &count)| count).unwrap();

            return Some(Scanner {
                id: format!("{},{}", self.id, other.id),
                scanners: self
                    .scanners
                    .iter()
                    .map(|s| rotate(s, &ROTATIONS[*idx]) - shift)
                    .chain(other.scanners.iter().copied())
                    .collect(),
                points: self
                    .points
                    .iter()
                    .map(|p| rotate(&p, &ROTATIONS[*idx]) - shift)
                    .chain(other.points.iter().cloned())
                    .collect(),
                angle_features: self
                    .angle_features
                    .iter()
                    .map(|(&k, v)| {
                        (
                            k,
                            v.iter()
                                .map(|p| rotate(p, &ROTATIONS[*idx]) - shift)
                                .collect::<HashSet<_>>(),
                        )
                    })
                    .chain(other.angle_features.iter().map(|(&k, v)| (k, v.clone())))
                    .collect(),
            });
        }

        None
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let detections = parse(&input)?;

    let mut arranged_pos: VecDeque<_> = detections
        .iter()
        .map(|(id, points)| Scanner::new(id.to_string(), points.iter().copied()))
        .collect();

    while arranged_pos.len() > 1 {
        dbg!(arranged_pos.len());
        let a = arranged_pos.pop_front().unwrap();
        loop {
            let b = arranged_pos.pop_front().unwrap();
            if let Some(arranged) = a.align_with_other(&b, 12) {
                arranged_pos.push_back(arranged);
                break;
            } else {
                arranged_pos.push_back(b);
            }
        }
    }
    dbg!(arranged_pos.len());

    let part1 = arranged_pos[0].points.len();
    dbg!(&part1);

    let part2 = arranged_pos[0]
        .scanners
        .iter()
        .combinations(2)
        .map(|vec| {
            let diff = vec[0] - vec[1];
            diff[0].abs() + diff[1].abs() + diff[2].abs()
        })
        .max();
    dbg!(&part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotation_has_identity() {
        let rotations = get_rotations();
        let v = Vector3::new(1, 2, 3);

        rotations
            .iter()
            .map(|r| rotate(&v, &r))
            .find(|&w| w == v)
            .unwrap();
    }
}
