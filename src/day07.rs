use crate::parse_custom_separated;

fn best_fuel_cost<F>(positions: &[i64], mut calc: F) -> i64
where
    F: FnMut(i64) -> i64,
{
    let (max, min) = (
        *positions.iter().max().unwrap(),
        *positions.iter().min().unwrap(),
    );

    let (_, fuel) = (min..=max)
        .map(|o| (o, positions.iter().map(|p| calc((p - o).abs())).sum()))
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap();

    fuel
}

fn identity<T>(x: T) -> T {
    x
}

pub fn star_one(input: &str) -> i64 {
    let positions: Vec<_> = parse_custom_separated::<i64>(input, ",").collect();

    best_fuel_cost(&positions, identity)
}

pub fn star_two(input: &str) -> i64 {
    let positions: Vec<_> = parse_custom_separated::<i64>(input, ",").collect();
    fn fuel_cost(moves: i64) -> i64 {
        (moves * (moves + 1)) / 2
    }

    best_fuel_cost(&positions, fuel_cost)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 37);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 168);
    }
}
