use crate::parse_lines;

fn count_increments(readings: impl IntoIterator<Item = u64>) -> u64 {
    readings
        .into_iter()
        .fold((0, None), |(total, previous_value), value| {
            if previous_value.map(|p| value > p).unwrap_or(false) {
                (total + 1, Some(value))
            } else {
                (total, Some(value))
            }
        })
        .0
}

pub fn star_one(input: &str) -> u64 {
    count_increments(parse_lines::<u64>(input))
}

pub fn star_two(input: &str) -> u64 {
    let readings: Vec<u64> = parse_lines(input).collect();

    count_increments(readings.windows(3).map(|readings| readings.iter().sum()))
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 7);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 5);
    }
}
