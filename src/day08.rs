use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Digit {
    value: usize,
    segments: HashSet<char>,
}

impl Digit {
    fn new(value: usize, segments: impl IntoIterator<Item = char>) -> Self {
        Self {
            value,
            segments: segments.into_iter().collect(),
        }
    }

    fn all() -> HashMap<usize, Self> {
        [
            Digit::new(0, "abcefg".chars()),
            Digit::new(1, "cf".chars()),
            Digit::new(2, "acdeg".chars()),
            Digit::new(3, "acdfg".chars()),
            Digit::new(4, "bcdf".chars()),
            Digit::new(5, "abdfg".chars()),
            Digit::new(6, "abdfeg".chars()),
            Digit::new(7, "acf".chars()),
            Digit::new(8, "abcdefg".chars()),
            Digit::new(9, "abcdfg".chars()),
        ]
        .into_iter()
        .map(|d| (d.value, d))
        .collect()
    }
}

// Map from number of segments to candidate digit
const SEGMENT_MAP: [(usize, usize); 10] = [
    (2, 1), // 1
    (4, 4), // 4
    (3, 7), // 7
    (7, 8), // 8
    // ---------------
    (6, 0), // 0
    (6, 9), // 9
    (6, 6), // 6
    // ---------------
    (5, 2), // 2
    (5, 3), // 3
    (5, 5), // 5
];

fn sorted_chars(chars: impl IntoIterator<Item = char>) -> Vec<char> {
    let mut result: Vec<_> = chars.into_iter().collect();

    result.sort();

    result
}

fn solve_line(segments: Vec<HashSet<char>>, outputs: &[String]) -> usize {
    let all_digits = Digit::all();

    let segment_map: HashMap<usize, Vec<Digit>> = {
        let mut result = HashMap::default();

        for (count, digit) in SEGMENT_MAP {
            let entry = result.entry(count).or_insert_with(|| Vec::with_capacity(1));

            entry.push(all_digits[&digit].clone());
        }

        result
    };

    let all_chars: HashSet<char> = segments
        .iter()
        .flat_map(|p| {
            p.iter()
                .filter(|char| char.is_ascii() && char.is_ascii_lowercase())
        })
        .map(|c| *c)
        .collect();

    let mut candidates: HashMap<char, HashSet<char>> =
        all_chars.iter().map(|c| (*c, all_chars.clone())).collect();

    let mut uniques: Vec<_> = segments
        .iter()
        .filter_map(|pattern| {
            segment_map
                .get(&pattern.len())
                .and_then(|c| (c.len() == 1).then(|| (pattern, &c[0])))
        })
        .collect();
    uniques.sort_by_key(|(_, u)| u.segments.len());

    // Unique lengths
    // (2, 1), // 1
    // (4, 4), // 4
    // (3, 7), // 7
    // (7, 8), // 8
    let one = uniques[0];
    let seven = uniques[1];
    let four = uniques[2];

    // Length 6
    // (6, 0), // 0
    // (6, 9), // 9
    // (6, 6), // 6
    let length_six: Vec<_> = segments
        .iter()
        .filter_map(|pattern| {
            segment_map
                .get(&pattern.len())
                .and_then(|c| (c[0].segments.len() == 6).then(|| (pattern, c)))
        })
        .collect();

    // (5, 2), // 2
    // (5, 3), // 3
    // (5, 5), // 5
    let length_five: Vec<_> = segments
        .iter()
        .filter_map(|pattern| {
            segment_map
                .get(&pattern.len())
                .and_then(|c| (c[0].segments.len() == 5).then(|| (pattern, c)))
        })
        .collect();

    let five_intersection = length_five
        .iter()
        .fold(length_five[0].0.clone(), |acc, (chars, _)| {
            acc.intersection(chars).copied().collect()
        });
    let six_intersection = length_six
        .iter()
        .fold(length_six[0].0.clone(), |acc, (chars, _)| {
            acc.intersection(chars).copied().collect()
        });

    candidates.insert('a', seven.0.difference(one.0).cloned().collect());
    candidates.insert(
        'd',
        four.0.intersection(&five_intersection).cloned().collect(),
    );
    candidates.insert(
        'f',
        one.0.intersection(&six_intersection).cloned().collect(),
    );
    candidates.insert('c', one.0.difference(&candidates[&'f']).cloned().collect());

    {
        let g: HashSet<_> = five_intersection.difference(&seven.0).cloned().collect();
        let g: HashSet<_> = g.difference(&candidates[&'d']).cloned().collect();
        let g = g.difference(&candidates[&'a']).copied().collect();

        candidates.insert('g', g);
    }

    {
        let b: HashSet<_> = four.0.difference(&one.0).cloned().collect();
        let b: HashSet<_> = b.difference(&candidates[&'d']).cloned().collect();

        candidates.insert('b', b);
    }
    {
        let mut e: HashSet<_> = all_chars;

        for c in candidates.keys() {
            if c != &'e' {
                let mapped_c = candidates[c].iter().nth(0).unwrap();
                e.remove(mapped_c);
            }
        }

        candidates.insert('e', e);
    }

    assert!(candidates.values().all(|c| c.len() == 1));

    let mapped_digits: HashMap<String, usize> = all_digits
        .values()
        .map(|d| {
            let mut mapped_segments: Vec<_> = d
                .segments
                .iter()
                .map(|c| candidates[c].iter().nth(0).unwrap())
                .copied()
                .collect();
            mapped_segments.sort();

            (mapped_segments.into_iter().collect(), d.value)
        })
        .collect();

    let num_digits = outputs.len();
    outputs.into_iter().enumerate().fold(0, |acc, (idx, s)| {
        acc + mapped_digits[s.as_str()] * 10_usize.pow((num_digits - idx - 1) as u32)
    })
}

pub fn star_one(input: &str) -> usize {
    let clean_lines = input.lines().map(str::trim).filter(|l| l.len() > 0);
    let parsed = clean_lines.map(|l| {
        let mut parts = l.split("|").map(str::trim);

        match (parts.next(), parts.next()) {
            (Some(patterns), Some(outputs)) => (
                patterns
                    .split_whitespace()
                    .map(|s| sorted_chars(s.chars()).into_iter().collect::<HashSet<_>>())
                    .collect(),
                outputs
                    .split_whitespace()
                    .map(|s| sorted_chars(s.chars()))
                    .map(|s| s.into_iter().collect())
                    .collect::<Vec<String>>(),
            ),
            _ => unreachable!("Each line must have two parts, not true for {}", l),
        }
    });

    parsed
        .map(|(patterns, outputs)| {
            let mut number = solve_line(patterns, &outputs);
            let mut exponent = ((number as f64).log10().floor()) as i32;
            let mut count = 0;

            while exponent >= 0 {
                let n = number / 10_usize.pow(exponent as u32);

                if n == 7 || n == 4 || n == 1 || n == 8 {
                    count += 1;
                }

                number = number % 10_usize.pow(exponent as u32);
                exponent -= 1;
            }

            count
        })
        .sum()
}

pub fn star_two(input: &str) -> usize {
    let clean_lines = input.lines().map(str::trim).filter(|l| l.len() > 0);
    let parsed = clean_lines.map(|l| {
        let mut parts = l.split("|").map(str::trim);

        match (parts.next(), parts.next()) {
            (Some(patterns), Some(outputs)) => (
                patterns
                    .split_whitespace()
                    .map(|s| sorted_chars(s.chars()).into_iter().collect::<HashSet<_>>())
                    .collect(),
                outputs
                    .split_whitespace()
                    .map(|s| sorted_chars(s.chars()))
                    .map(|s| s.into_iter().collect())
                    .collect::<Vec<String>>(),
            ),
            _ => unreachable!("Each line must have two parts, not true for {}", l),
        }
    });

    parsed
        .map(|(patterns, outputs)| solve_line(patterns, &outputs))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_line, star_one, star_two};
    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 26);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 61229);
    }

    #[test]
    fn test_solve_line() {
        let segments = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        assert_eq!(
            solve_line(
                segments,
                &vec![
                    "bcdef".into(),
                    "abcdf".into(),
                    "bcdef".into(),
                    "abcdf".into()
                ]
            ),
            5353
        );
    }

    #[test]
    fn test_solve_line_two() {
        let segments = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        assert_eq!(
            solve_line(
                segments,
                &vec![
                    "abcdefg".into(),
                    "bcdef".into(),
                    "bcdefg".into(),
                    "bceg".into()
                ]
            ),
            8394
        );
    }
}
