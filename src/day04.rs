use core::fmt;
use std::str::FromStr;

use crate::parse_custom_separated;

#[derive(Debug, Clone)]
struct Entry {
    number: u64,
    marked: bool,
}

impl From<u64> for Entry {
    fn from(number: u64) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<Entry>>,
}

impl Board {
    fn mark(&mut self, number: u64) {
        for row in &mut self.rows {
            for entry in row {
                if !entry.marked {
                    entry.marked = entry.number == number;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        let mut range = 0..self.rows.len();

        let is_row_win = range
            .clone()
            .any(|row_idx| self.rows[row_idx].iter().all(|e| e.marked));

        let is_column_win = range.any(|col_idx| self.rows.iter().all(|row| row[col_idx].marked));

        is_row_win || is_column_win
    }

    fn all_entries(&self) -> impl Iterator<Item = &Entry> {
        self.rows.iter().flat_map(|row| row.iter())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for e in row {
                if e.marked {
                    write!(f, "*{}\t", e.number)?;
                } else {
                    write!(f, "{}\t", e.number)?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<Entry>> = s
            .lines()
            .map(str::trim)
            .filter(|l| l.len() > 0)
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u64>().expect("Should be parasable number"))
                    .map(From::from)
                    .collect()
            })
            .collect();

        assert!(rows.len() == 5 && rows.iter().all(|row| row.len() == 5));

        Ok(Self { rows })
    }
}

fn play(
    draws: impl IntoIterator<Item = u64>,
    boards: &mut [Board],
    first_win: bool,
) -> (Board, u64) {
    let mut won_boards = 0;
    let board_count = boards.len();

    for draw in draws {
        for board in &mut *boards {
            if board.is_win() {
                continue;
            }

            board.mark(draw);

            if board.is_win() {
                if first_win {
                    return (board.clone(), draw);
                } else {
                    won_boards += 1;

                    if won_boards == board_count {
                        return (board.clone(), draw);
                    }
                }
            }
        }
    }

    unreachable!()
}

pub fn star_one(input: &str) -> u64 {
    let draws = parse_custom_separated::<u64>(input.lines().nth(0).unwrap(), ",");

    let mut boards: Vec<Board> = input
        .split("\n\n")
        .skip(1)
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, String>>()
        .expect("Failed to parse boards");

    let (winning_board, last_draw) = play(draws, &mut boards, true);

    winning_board
        .all_entries()
        .filter_map(|e| (!e.marked).then(|| e.number))
        .sum::<u64>()
        * last_draw
}

pub fn star_two(input: &str) -> u64 {
    let draws = parse_custom_separated::<u64>(input.lines().nth(0).unwrap(), ",");

    let mut boards: Vec<Board> = input
        .split("\n\n")
        .skip(1)
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, String>>()
        .expect("Failed to parse boards");

    let (winning_board, last_draw) = play(draws, &mut boards, false);

    winning_board
        .all_entries()
        .filter_map(|e| (!e.marked).then(|| e.number))
        .sum::<u64>()
        * last_draw
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 4512);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 1924);
    }
}
