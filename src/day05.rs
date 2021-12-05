use std::{collections::HashMap, str::FromStr};

use crate::parse_lines;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0 };

    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        fn parse(value: Option<&str>) -> Result<i64, String> {
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
    start: Point,
    end: Point,
}

impl Line {
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

    fn contains(&self, point: &Point) -> bool {
        if self.is_horizontal() || self.is_vertical() {
            (point.x >= self.start.x.min(self.end.x) && point.x <= self.end.x.max(self.start.x))
                && (point.y >= self.start.y.min(self.end.y)
                    && point.y <= self.end.y.max(self.start.y))
        } else if self.is_diagonal() {
            let m = (self.start.y - self.end.y) / (self.start.x - self.end.x);

            let on_line = (point.y - self.start.y) == m * (point.x - self.start.x);
            let between_points = point.x >= self.start.x.min(self.end.x)
                && point.x <= self.start.x.max(self.end.x)
                && point.y >= self.start.y.min(self.end.y)
                && point.y <= self.start.y.max(self.end.y);

            on_line && between_points
        } else {
            false
        }
    }

    fn points(&self) -> LinePointsIterator {
        let step_x = (self.end.x - self.start.x).clamp(-1, 1);
        let step_y = (self.end.y - self.start.y).clamp(-1, 1);

        LinePointsIterator {
            current: self.start,
            step: Point::new(step_x, step_y),
            end: self.end,
            done: false,
        }
    }
}

struct LinePointsIterator {
    current: Point,
    step: Point,
    end: Point,
    done: bool,
}

impl Iterator for LinePointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.current == self.end {
            self.done = true;
        }

        let next = self.current;
        self.current = Point::new(self.current.x + self.step.x, self.current.y + self.step.y);

        Some(next)
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->");

        fn parse(value: Option<&str>) -> Result<Point, String> {
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
            (Ok(start), Ok(end)) => Ok(Self { start, end }),
            _ => Err(format!("Invalid line {}", s)),
        }
    }
}

fn count(lines: impl IntoIterator<Item = Line>) -> usize {
    let counts = lines.into_iter().flat_map(|l| l.points()).fold(
        HashMap::<Point, usize>::default(),
        |mut acc, point| {
            (*acc.entry(point).or_insert(0)) += 1;

            acc
        },
    );

    counts.into_iter().filter(|(_, c)| c >= &2).count()
}

pub fn star_one(input: &str) -> usize {
    let lines: Vec<Line> = parse_lines(input).collect();

    count(
        lines
            .into_iter()
            .filter(|l| l.is_horizontal() || l.is_vertical()),
    )
}

pub fn star_two(input: &str) -> usize {
    let lines: Vec<Line> = parse_lines(input).collect();

    count(
        lines
            .into_iter()
            .filter(|l| l.is_horizontal() || l.is_vertical() || l.is_diagonal()),
    )
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
    fn test_line_contains() {
        let line = Line {
            start: Point::new(0, 9),
            end: Point::new(5, 9),
        };

        assert!(line.contains(&Point::new(0, 9)));
    }

    #[test]
    fn test_line_contains_diagonal() {
        let line = Line {
            start: Point::new(1, 1),
            end: Point::new(3, 3),
        };

        assert!(line.contains(&Point::new(1, 1)));
        assert!(line.contains(&Point::new(2, 2)));
        assert!(line.contains(&Point::new(3, 3)));
    }

    #[test]
    fn test_line_contains_diagonal_negative() {
        let line = Line {
            start: Point::new(1, 3),
            end: Point::new(3, 1),
        };

        assert!(line.contains(&Point::new(1, 3)));
        assert!(line.contains(&Point::new(2, 2)));
        assert!(line.contains(&Point::new(3, 1)));
    }

    #[test]
    fn test_line_contains_diagonal_1() {
        let line = Line {
            start: Point::new(8, 0),
            end: Point::new(0, 8),
        };

        assert!(!line.contains(&Point::new(0, 0)));
    }

    #[test]
    fn test_points_diagonal() {
        let line = Line {
            start: Point::new(1, 3),
            end: Point::new(3, 1),
        };

        let points: Vec<_> = line.points().collect();

        assert_eq!(
            points,
            vec![Point::new(1, 3), Point::new(2, 2), Point::new(3, 1),]
        );
    }

    #[test]
    fn test_points_horizontal() {
        let line = Line {
            start: Point::new(0, 3),
            end: Point::new(3, 3),
        };

        let points: Vec<_> = line.points().collect();

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
