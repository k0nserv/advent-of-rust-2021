use std::{collections::HashMap, str::FromStr};

use crate::parse_lines;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    const ORIGIN: Point<i64> = Point { x: 0, y: 0 };

    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<f64> {
    fn normalize(&self) -> Self {
        let length = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();

        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }
}

impl<T: FromStr> FromStr for Point<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        fn parse<V: FromStr>(value: Option<&str>) -> Result<V, String>
        where
            <V as FromStr>::Err: std::fmt::Display,
        {
            let invalid_point = || format!("Invalid point {:?}", value);

            value
                .ok_or_else(invalid_point)
                .map(str::trim)
                .and_then(|s| {
                    s.parse()
                        .map_err(|e| format!("Faild to parse point {}, with error: {}", s, e))
                })
        }

        match (parse(parts.next()), parse(parts.next())) {
            (Ok(x), Ok(y)) => Ok(Self { x, y }),
            _ => Err(format!("Invalid point {}", s)),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point<i64>,
    end: Point<i64>,
    dir: Point<f64>,
    step: Point<f64>,
}

impl Line {
    fn new(start: Point<i64>, end: Point<i64>) -> Self {
        fn fix_step(s: f64) -> f64 {
            if s.is_infinite() {
                0.0
            } else {
                s.abs()
            }
        }

        let dir =
            Point::new(end.x as f64 - start.x as f64, end.y as f64 - start.y as f64).normalize();

        let step = Point::new(fix_step(1.0 / dir.x), fix_step(1.0 / dir.y));

        Self {
            start,
            end,
            dir,
            step,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        let x_diff = self.start.x.max(self.end.x) - self.end.x.min(self.start.x);
        let y_diff = self.start.y.max(self.end.y) - self.end.y.min(self.start.y);

        x_diff == y_diff
    }

    fn points(&self) -> LinePointsIterator {
        LinePointsIterator {
            current: self.start,
            dir: self.dir,
            step: self.step,
            end: self.end,
            done: false,
        }
    }
}

struct LinePointsIterator {
    current: Point<i64>,
    dir: Point<f64>,
    step: Point<f64>,
    end: Point<i64>,
    done: bool,
}

impl Iterator for LinePointsIterator {
    type Item = Point<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.current == self.end {
            self.done = true;
        }

        let next = self.current;
        self.current = Point::new(
            (self.current.x as f64 + self.step.x * self.dir.x).round() as i64,
            (self.current.y as f64 + self.step.y * self.dir.y).round() as i64,
        );

        Some(next)
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->");

        fn parse(value: Option<&str>) -> Result<Point<i64>, String> {
            let invalid_point = || format!("Invalid point {:?}", value);

            value
                .ok_or_else(invalid_point)
                .map(str::trim)
                .and_then(|s| {
                    s.parse()
                        .map_err(|e| format!("Faild to parse point {}, with error: {}", s, e))
                })
        }

        match (parse(parts.next()), parse(parts.next())) {
            (Ok(start), Ok(end)) => Ok(Self::new(start, end)),
            _ => Err(format!("Invalid line {}", s)),
        }
    }
}

fn count(lines: impl IntoIterator<Item = Line>) -> usize {
    let counts = lines.into_iter().flat_map(|l| l.points()).fold(
        HashMap::<Point<i64>, usize>::default(),
        |mut acc, point| {
            (*acc.entry(point).or_insert(0)) += 1;

            acc
        },
    );

    counts.into_iter().filter(|(_, c)| c >= &2).count()
}

pub fn star_one(input: &str) -> usize {
    let lines = parse_lines::<Line>(input);

    count(lines.filter(|l| l.is_horizontal() || l.is_vertical()))
}

pub fn star_two(input: &str) -> usize {
    let lines = parse_lines::<Line>(input);

    count(lines.filter(|l| l.is_horizontal() || l.is_vertical() || l.is_diagonal()))
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, Line, Point};

    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 5);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 12);
    }

    #[test]
    fn test_points_diagonal() {
        let line = Line::new(Point::new(1, 3), Point::new(3, 1));

        let points: Vec<_> = line.points().collect();

        assert_eq!(
            points,
            vec![Point::new(1, 3), Point::new(2, 2), Point::new(3, 1),]
        );
    }

    #[test]
    fn test_points_horizontal() {
        let line = Line::new(Point::new(0, 3), Point::new(3, 3));

        let points: Vec<_> = line.points().take(5).collect();

        assert_eq!(
            points,
            vec![
                Point::new(0, 3),
                Point::new(1, 3),
                Point::new(2, 3),
                Point::new(3, 3)
            ]
        );
    }
}
