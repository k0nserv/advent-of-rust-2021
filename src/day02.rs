use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("Invalid direction {}", s)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    units: i64,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();

        let direction: Direction = match parts.next().map(str::trim) {
            Some(s) => s.parse()?,
            _ => return Err(format!("Invalid instruction {}", s)),
        };

        let units = match parts.next().map(str::trim).map(str::parse) {
            Some(Ok(units)) => units,
            _ => return Err(format!("Invalid instruction {}", s)),
        };

        Ok(Instruction { direction, units })
    }
}

pub fn star_one(input: &str) -> i64 {
    let instructions = parse_lines::<Instruction>(input);

    let final_pos = instructions.fold((0, 0), |pos, instruction| match instruction.direction {
        Direction::Forward => (pos.0 + instruction.units, pos.1),
        Direction::Down => (pos.0, pos.1 + instruction.units),
        Direction::Up => (pos.0, pos.1 - instruction.units),
    });

    final_pos.0 * final_pos.1
}

pub fn star_two(input: &str) -> i64 {
    let instructions = parse_lines::<Instruction>(input);

    let final_pos = instructions.fold((0, 0, 0), |pos, instruction| match instruction.direction {
        Direction::Forward => (
            pos.0 + instruction.units,
            pos.1 + instruction.units * pos.2,
            pos.2,
        ),
        Direction::Down => (pos.0, pos.1, pos.2 + instruction.units),
        Direction::Up => (pos.0, pos.1, pos.2 - instruction.units),
    });

    final_pos.0 * final_pos.1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 150);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 900);
    }
}
