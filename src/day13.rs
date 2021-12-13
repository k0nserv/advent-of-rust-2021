use std::{collections::HashSet, str::FromStr};

use crate::parse_custom_separated;

#[derive(PartialEq, Eq)]
enum Axis {
    Horizontal,
    Vertical,
}

impl FromStr for Axis {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::Horizontal),
            "y" => Ok(Self::Vertical),
            _ => Err(format!("Invalid axis: {}", s)),
        }
    }
}

fn location_around_fold(loc: (u64, u64), fold: &(Axis, u64)) -> (u64, u64) {
    let (axis, fold_location) = fold;

    if axis == &Axis::Horizontal {
        if loc.0 < *fold_location {
            loc
        } else {
            (
                (fold_location - loc.0 % fold_location) % fold_location,
                loc.1,
            )
        }
    } else {
        if loc.1 < *fold_location {
            loc
        } else {
            (loc.0, (fold.1 - loc.1 % fold_location) % fold_location)
        }
    }
}

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = (u64, u64)> + '_,
    impl Iterator<Item = (Axis, u64)> + '_,
) {
    let mut parts = input.trim().split("\n\n");
    let grid = parts
        .next()
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut iter = parse_custom_separated::<u64>(l, ",");

            (iter.next().unwrap(), iter.next().unwrap())
        });

    let folds = parts
        .next()
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|s| {
            let rest = s.strip_prefix("fold along ").unwrap();
            let mut parts = rest.split("=");

            match (parts.next().map(str::trim), parts.next().map(str::trim)) {
                (Some(axis), Some(value)) => {
                    let parsed_axis: Axis = axis
                        .parse()
                        .expect(&format!("Failed to parse axis in: {}", s));

                    (
                        parsed_axis,
                        value
                            .parse::<u64>()
                            .expect(&format!("Failed to parse fold location in: {}", s)),
                    )
                }
                _ => unreachable!(),
            }
        });

    (grid, folds)
}

fn print_grid(grid: HashSet<(u64, u64)>) {
    let max = grid
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));
    dbg!(max);

    let rows = (0..=max.1)
        .map(|y| {
            (0..=max.0)
                .map(|x| {
                    if grid.contains(&(x, y)) {
                        "\u{2588}"
                    } else {
                        " "
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    println!("{}", rows.join("\n"));
}

pub fn star_one(input: &str) -> usize {
    let (grid, mut folds) = parse_input(input);

    let first_fold = folds.nth(0).unwrap();

    let locations: HashSet<_> = grid
        .map(|loc| location_around_fold(loc, &first_fold))
        .collect();

    locations.len()
}

pub fn star_two(input: &str) -> &str {
    let (grid, folds) = parse_input(input);
    let mut grid: HashSet<_> = grid.collect();

    for fold in folds {
        grid = grid
            .into_iter()
            .map(|loc| location_around_fold(loc, &fold))
            .collect();
    }

    print_grid(grid);

    "CPJBERVL"
}

#[cfg(test)]
mod tests {
    use super::star_one;
    const INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 17);
    }
}
