use std::mem;

use crate::parse_custom_separated;

fn tick(fishes: &mut [usize], new_fishes: &mut [usize]) {
    for key in 0..=8 {
        if key == 0 {
            new_fishes[6] += fishes[0];
            new_fishes[8] += fishes[0];
        } else {
            new_fishes[key - 1] += fishes[key];
        }
    }
}

fn run(input: &str, tick_count: usize) -> usize {
    let mut fish: [usize; 9] = {
        let mut result: [usize; 9] = [0; 9];

        for timer in parse_custom_separated::<usize>(input, ",") {
            result[timer] += 1;
        }

        result
    };
    let mut new_fishes: [usize; 9] = [0; 9];

    for _ in 0..tick_count {
        tick(&mut fish, &mut new_fishes);
        fish = [0; 9];
        mem::swap(&mut fish, &mut new_fishes);
    }

    if tick_count % 2 == 0 {
        fish.into_iter().sum()
    } else {
        new_fishes.into_iter().sum()
    }
}

pub fn star_one(input: &str) -> usize {
    run(input, 80)
}

pub fn star_two(input: &str) -> usize {
    run(input, 256)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 5934);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 26984457539);
    }
}
