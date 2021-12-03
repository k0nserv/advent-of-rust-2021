const fn mask(n: usize) -> u64 {
    !(!0_u64 << n)
}

#[derive(Copy, Clone)]
enum Mode {
    MostCommon,
    LeastCommon,
}

fn find_bit(x: usize, data: &[Vec<char>], mode: Mode) -> u8 {
    let (ones, zeroes) = (0..data.len()).fold((0, 0), |(count_ones, count_zeroes), y| {
        if data[y][x] == '1' {
            (count_ones + 1, count_zeroes)
        } else {
            (count_ones, count_zeroes + 1)
        }
    });

    match mode {
        Mode::MostCommon => (ones >= zeroes) as u8,
        Mode::LeastCommon => (ones < zeroes) as u8,
    }
}

fn find_rating(mut data: Vec<Vec<char>>, mode: Mode) -> u64 {
    let mut x = 0;

    while data.len() > 1 {
        let bit = find_bit(x, &data, mode);
        let mut should_retain = data
            .iter()
            .map(|row| row[x] == '1' && bit == 1 || row[x] == '0' && bit == 0)
            .collect::<Vec<_>>()
            .into_iter();

        data.retain(|_| should_retain.next().unwrap());

        x += 1;
    }

    let length = data[0].len();
    data[0].iter().enumerate().fold(0_u64, |acc, (idx, bit)| {
        let numeric_value = match bit {
            '1' => 1,
            '0' => 0,
            _ => unreachable!(),
        };

        acc | (numeric_value << length - idx - 1)
    })
}

pub fn star_one(input: &str) -> u64 {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();

    let number_length = data[0].len();
    let gamma_rate = (0..number_length).fold(0_u64, |acc, x| {
        let bit = find_bit(x, &data, Mode::MostCommon);

        acc | ((bit as u64) << (number_length - x - 1))
    });
    let epsilon_rate = (gamma_rate ^ u64::MAX) & mask(number_length);

    gamma_rate * epsilon_rate
}

pub fn star_two(input: &str) -> u64 {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();

    let oxygen_rating = find_rating(data.clone(), Mode::MostCommon);
    let co2_scrubber_rating = find_rating(data.clone(), Mode::LeastCommon);

    oxygen_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 198);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 230);
    }
}
