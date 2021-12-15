use std::{collections::HashMap, mem};

fn estimate_size(template_length: usize, steps: usize) -> usize {
    let mut n = template_length;

    for _ in 0..steps {
        n = n + n - 1;
    }

    n
}

// This is how I solved part 1, which clearly doens't work for part 2.
fn expand(
    input: &str,
    pairs: &HashMap<(char, char), char>,
    steps: usize,
) -> impl Iterator<Item = char> {
    let size = estimate_size(input.len(), steps);
    let mut result = Vec::with_capacity(size);
    let mut other_result = Vec::with_capacity(size);
    result.extend(input.chars());
    result.resize(size, '.');
    other_result.resize(size, '.');

    for _ in 0..steps {
        for (idx, chars) in result.windows(2).enumerate() {
            if chars[1] == '.' {
                break;
            }

            let sub = pairs[&(chars[0], chars[1])];
            other_result[idx * 2] = chars[0];
            other_result[idx * 2 + 1] = sub;
            other_result[idx * 2 + 2] = chars[1];
        }

        mem::swap(&mut result, &mut other_result);
    }

    result.into_iter()
}

fn expand_smort(
    input: &str,
    pairs: &HashMap<(char, char), char>,
    steps: usize,
) -> impl Iterator<Item = ((char, char), usize)> {
    let template: Vec<_> = input.chars().collect();
    let mut result: HashMap<_, _> = pairs.keys().map(|k| (*k, 0)).collect();

    for chars in template.windows(2) {
        *result.get_mut(&(chars[0], chars[1])).unwrap() += 1;
    }
    let mut other_result = result.clone();

    for _ in 0..steps {
        other_result.iter_mut().for_each(|(_, v)| *v = 0);

        for (chars, count) in &result {
            let sub = pairs[&(chars.0, chars.1)];

            *other_result.get_mut(&(chars.0, sub)).unwrap() += count;
            *other_result.get_mut(&(sub, chars.1)).unwrap() += count;
        }
        mem::swap(&mut result, &mut other_result);
    }

    result.into_iter()
}

fn parse(input: &str) -> (&str, HashMap<(char, char), char>) {
    let mut parts = input.split("\n\n").map(str::trim);
    let template = parts.next().unwrap();
    let pairs: HashMap<_, _> = parts
        .next()
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut parts = l.split("->").map(str::trim);

            let pattern = parts.next().unwrap();
            let (p1, p2) = (
                pattern.chars().next().unwrap(),
                pattern.chars().skip(1).next().unwrap(),
            );
            let substitution = parts.next().unwrap();

            ((p1, p2), substitution.chars().next().unwrap())
        })
        .collect();

    (template, pairs)
}

fn count(
    result: impl IntoIterator<Item = ((char, char), usize)>,
    last: char,
) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> =
        result
            .into_iter()
            .fold(HashMap::new(), |mut acc, ((c1, _), count)| {
                *acc.entry(c1).or_default() += count;
                acc
            });

    // Whather the last character is it will have been under counted by one, fix it.
    *counts.get_mut(&last).unwrap() += 1;

    counts
}

pub fn star_one(input: &str) -> usize {
    let (template, pairs) = parse(input);
    let result = expand_smort(template, &pairs, 10);
    let last = template.chars().last().unwrap();

    let counts = count(result, last);

    let max = counts.values().max_by_key(|&count| count).unwrap();
    let min = counts.values().min_by_key(|&count| count).unwrap();

    max - min
}

pub fn star_two(input: &str) -> usize {
    let (template, pairs) = parse(input);
    let result = expand_smort(template, &pairs, 40);
    let last = template.chars().last().unwrap();

    let counts = count(result, last);

    let max = counts.values().max_by_key(|&count| count).unwrap();
    let min = counts.values().min_by_key(|&count| count).unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 1588);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 2188189693529);
    }
}
