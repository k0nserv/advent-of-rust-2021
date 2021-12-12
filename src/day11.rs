use std::{collections::HashSet, mem};

type State = Vec<Vec<u32>>;
type StateRef<'a> = &'a [Vec<u32>];
type MutStateRef<'a> = &'a mut [Vec<u32>];

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

fn neighbors(p: (usize, usize), max: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.iter().flat_map(move |dir| {
        let point = (p.0 as isize + dir.0, p.1 as isize + dir.1);

        if point.0 < 0
            || point.1 < 0
            || (point.0) > (max.0 as isize - 1)
            || (point.1) > (max.1 as isize - 1)
        {
            return None;
        }

        Some((point.0 as usize, point.1 as usize))
    })
}

fn tick(state: StateRef, next_state: MutStateRef, max: (usize, usize)) -> usize {
    let mut did_flash: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..max.1 {
        for x in 0..max.0 {
            next_state[y][x] = state[y][x] + 1;
        }
    }

    loop {
        let should_flash = next_state.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &o)| {
                if did_flash.contains(&(x, y)) {
                    return None;
                }

                (o > 9).then(|| (x, y))
            })
        });

        let location = match should_flash {
            None => break,
            Some(l) => l,
        };

        did_flash.insert(location);
        let neighbors = neighbors(location, max);

        for (x, y) in neighbors {
            next_state[y][x] += 1;
        }
    }

    for (x, y) in &did_flash {
        next_state[*y][*x] = 0;
    }

    did_flash.len()
}

pub fn star_one(input: &str) -> usize {
    let mut state: State = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut other_state = state.clone();
    let max = (state[0].len(), state.len());

    (0..100).fold(0, |acc, _| {
        let flahses = tick(&state, &mut other_state, max);
        mem::swap(&mut state, &mut other_state);

        acc + flahses
    })
}

pub fn star_two(input: &str) -> usize {
    let mut state: State = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut other_state = state.clone();
    let max = (state[0].len(), state.len());

    let step = (0..)
        .find_map(|s| {
            let flahses = tick(&state, &mut other_state, max);
            mem::swap(&mut state, &mut other_state);

            if flahses == state.len() * state[0].len() {
                Some(s + 1)
            } else {
                None
            }
        })
        .unwrap();

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 1656);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 195);
    }

    #[test]
    fn test_tick() {
        let input = r#"11111
19991
19191
19991
11111"#;

        let state: State = input
            .lines()
            .map(str::trim)
            .filter(|l| l.len() > 0)
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let mut other_state = state.clone();
        let max = (state[0].len(), state.len());

        let result = tick(&state, &mut &mut other_state, max);

        assert_eq!(result, 9);
    }
}
