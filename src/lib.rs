#![allow(dead_code)]
#[allow(unused_variables)]
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code, unused_imports)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod math;

#[derive(Debug, Copy, Clone)]
pub struct DigitIterator {
    initial_value_is_zero: bool,
    number: f64,
}

impl DigitIterator {
    fn new(number: usize) -> Self {
        Self {
            initial_value_is_zero: number == 0,
            number: number as f64,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number < 1.0 && !self.initial_value_is_zero {
            return None;
        }

        if self.initial_value_is_zero {
            self.initial_value_is_zero = false;

            Some(0)
        } else {
            let digit = self.number % 10_f64;
            self.number = (self.number / 10_f64).floor();

            Some(digit as usize)
        }
    }
}

fn time<F>(label: &str, closure: F)
where
    F: Fn(),
{
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    closure();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    println!(
        "Time taken for {}: {}s and {}ns",
        label,
        time.as_secs(),
        time.subsec_nanos()
    );
}

/// Parse lines of text into custom types.
///
/// Each line is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse whitespace separated custom types.
///
/// Each unit separated by whitespace is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_whitespace_separated<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split_whitespace()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse custom separator separated custom types.
///
/// Each unit separated by a specific separator is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_custom_separated<'a, T>(
    input: &'a str,
    separator: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split(separator)
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

pub fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    #[test]
    fn solve_day01() {
        use crate::day01::{star_one, star_two};

        let input = load_file("day01.txt");

        assert_eq!(star_one(&input), 1624);
        assert_eq!(star_two(&input), 1653);
    }

    #[test]
    fn solve_day02() {
        use crate::day02::{star_one, star_two};

        let input = load_file("day02.txt");

        assert_eq!(star_one(&input), 2073315);
        assert_eq!(star_two(&input), 1840311528);
    }

    #[test]
    fn solve_day03() {
        use crate::day03::{star_one, star_two};

        let input = load_file("day03.txt");

        assert_eq!(star_one(&input), 4147524);
        assert_eq!(star_two(&input), 3570354);
    }

    #[test]
    fn solve_day04() {
        use crate::day04::{star_one, star_two};

        let input = load_file("day04.txt");

        assert_eq!(star_one(&input), 29440);
        assert_eq!(star_two(&input), 13884);
    }

    #[test]
    fn solve_day05() {
        use crate::day05::{star_one, star_two};

        let input = load_file("day05.txt");

        super::time("Day 5, part 1", || assert_eq!(star_one(&input), 5169));
        super::time("Day 5, part 2", || assert_eq!(star_two(&input), 22083));
    }

    #[test]
    fn solve_day06() {
        use crate::day06::{star_one, star_two};

        let input = load_file("day06.txt");

        super::time("Day 6, part 1", || assert_eq!(star_one(&input), 350605));
        super::time("Day 6, part 2", || {
            assert_eq!(star_two(&input), 1592778185024)
        });
    }

    #[test]
    fn solve_day07() {
        use crate::day07::{star_one, star_two};

        let input = load_file("day07.txt");

        assert_eq!(star_one(&input), 356958);
        assert_eq!(star_two(&input), 105461913);
    }

    #[test]
    fn solve_day08() {
        use crate::day08::{star_one, star_two};

        let input = load_file("day08.txt");

        assert_eq!(star_one(&input), 330);
        assert_eq!(star_two(&input), 1010472);
    }

    #[test]
    fn solve_day09() {
        use crate::day09::{star_one, star_two};

        let input = load_file("day09.txt");

        assert_eq!(star_one(&input), 633);
        assert_eq!(star_two(&input), 1050192);
    }

    #[test]
    fn solve_day10() {
        use crate::day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), 344193);
        assert_eq!(star_two(&input), 3241238967);
    }

    #[test]
    fn solve_day11() {
        use crate::day11::{star_one, star_two};

        let input = load_file("day11.txt");

        assert_eq!(star_one(&input), 1749);
        assert_eq!(star_two(&input), 285);
    }

    #[test]
    fn solve_day12() {
        use crate::day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 3887);
        assert_eq!(star_two(&input), 104834);
    }

    #[test]
    fn solve_day13() {
        use crate::day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 724);
        assert_eq!(star_two(&input), "CPJBERVL");
    }

    #[test]
    fn solve_day14() {
        use crate::day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 3259);
        assert_eq!(star_two(&input), 3459174981021);
    }

    #[test]
    fn solve_day15() {
        use crate::day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 687);
        assert_eq!(star_two(&input), 2957);
    }

    #[test]
    fn solve_day16() {
        use crate::day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 901);
        assert_eq!(star_two(&input), 110434737925);
    }

    #[test]
    fn solve_day17() {
        use crate::day17::{star_one, star_two};

        assert_eq!(star_one((94, -156), (151, -103)), 12090);
        assert_eq!(star_two((94, -156), (151, -103)), 5059);
    }

    #[test]
    fn solve_day18() {
        use crate::day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day19() {
        use crate::day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day20() {
        use crate::day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day21() {
        use crate::day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day22() {
        use crate::day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day23() {
        use crate::day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day24() {
        use crate::day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}
