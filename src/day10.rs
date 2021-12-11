use std::collections::VecDeque;

trait CharExt {
    fn is_opening(&self) -> bool;
    fn is_closing(&self) -> bool;

    fn closing_for_open(&self) -> Self;

    fn score_corrupted(&self) -> u64;
    fn score_uncorrupted(&self) -> u64;
}

impl CharExt for char {
    fn is_opening(&self) -> bool {
        *self == '{' || *self == '<' || *self == '[' || *self == '('
    }

    fn is_closing(&self) -> bool {
        *self == '}' || *self == '>' || *self == ']' || *self == ')'
    }

    fn closing_for_open(&self) -> Self {
        match self {
            '{' => '}',
            '[' => ']',
            '(' => ')',
            '<' => '>',
            _ => unreachable!(),
        }
    }

    //     ): 3 points.
    //     ]: 57 points.
    //     }: 1197 points.
    //     >: 25137 points.
    fn score_corrupted(&self) -> u64 {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }

    fn score_uncorrupted(&self) -> u64 {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        }
    }
}

fn find_first_illegal(line: &str) -> Result<char, VecDeque<char>> {
    let mut stack = VecDeque::<char>::new();

    for c in line.chars() {
        assert!(c.is_opening() || c.is_closing());

        if c.is_opening() {
            stack.push_front(c);
        } else {
            let popped = stack.pop_front().unwrap();

            if popped.closing_for_open() != c {
                return Ok(c);
            }
        }
    }

    Err(stack)
}
pub fn star_one(input: &str) -> u64 {
    let result = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .filter_map(|l| find_first_illegal(l).ok());

    result.map(|c| c.score_corrupted()).sum()
}

pub fn star_two(input: &str) -> u64 {
    let scores: Vec<_> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .filter_map(|l| find_first_illegal(l).err())
        .map(|stack| {
            stack.into_iter().fold(0, |acc, c| {
                let closing = c.closing_for_open();
                let score = closing.score_uncorrupted();

                (acc * 5) + score
            })
        })
        .collect();

    let sorted_scores = {
        let mut scores = scores;
        scores.sort();

        scores
    };

    sorted_scores[sorted_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 26397);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 288957);
    }
}
