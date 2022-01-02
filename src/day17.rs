use std::collections::HashSet;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    const ORIGIN: Point<usize> = Point { x: 0, y: 0 };

    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default + PartialEq + Eq> Point<T> {
    fn is_origin(&self) -> bool {
        self.x == T::default() && self.y == T::default()
    }
}

struct Area {
    min: Point<i64>,
    max: Point<i64>,
}

impl Area {
    fn contains(&self, p: &Point<i64>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

type Path = Vec<Point<i64>>;

/// Calculate the number of steps needed to be simulated to know if the target area has been
/// reached.
///
/// Assumes that velocity.y is positive
fn calculate_steps_required(y_velocity: i64, target_area: &Area) -> usize {
    // Number of steps to get back to origin
    let steps_to_origin = (y_velocity * 2 + 1) as usize;

    let mut velocity = -y_velocity;
    let mut y_pos = 0;
    let mut extra_steps = 0;

    while y_pos > target_area.min.y {
        y_pos += velocity;
        velocity -= 1;

        extra_steps += 1;
    }

    steps_to_origin + extra_steps
}

fn calculate_new_velocity(current: Point<i64>) -> Point<i64> {
    let new_x_velocity = current.x + current.x.signum() * -1;

    Point::new(new_x_velocity, current.y - 1)
}

fn simulate(
    from: Point<i64>,
    initial_velocity: Point<i64>,
    steps: usize,
    area: &Area,
) -> Option<(Point<i64>, Path)> {
    if initial_velocity.x > area.max.x && initial_velocity.y > area.max.y {
        // There's no way
        return None;
    }

    let mut path = Vec::with_capacity(steps);

    let mut position = from;
    let mut velocity = initial_velocity;

    for _ in 0..steps {
        position = Point::new(position.x + velocity.x, position.y + velocity.y);
        path.push(position);

        velocity = calculate_new_velocity(velocity);

        if area.contains(&position) {
            return Some((initial_velocity, path));
        }

        if velocity.y < 0 && position.y < area.min.y {
            // We are falling below the target area and will thus never hit it
            return None;
        } else if velocity.x == 0 && (position.x < area.min.x || position.x > area.max.x) {
            // We are falling straight down to the right or left of the target area straight down
            return None;
        }
    }

    None
}

pub fn star_one(min: (i64, i64), max: (i64, i64)) -> i64 {
    let area = Area {
        min: Point::new(min.0, min.1),
        max: Point::new(max.0, max.1),
    };
    let area_ref = &area;
    let mut results = vec![];

    for y in min.1..1000 {
        let steps = calculate_steps_required(y, &area);
        for x in 0..1000 {
            if let Some(path) = simulate(Point::new(0, 0), Point::new(x, y), steps, area_ref) {
                results.push(path);
            }
        }
    }

    let max_y = results
        .into_iter()
        .map(|(_, path)| {
            let highest_point = path.iter().max_by_key(|p| p.y).copied().unwrap();

            highest_point.y
        })
        .max()
        .unwrap();

    max_y
}

pub fn star_two(min: (i64, i64), max: (i64, i64)) -> usize {
    let area = Area {
        min: Point::new(min.0, min.1),
        max: Point::new(max.0, max.1),
    };

    let mut unique_velocities: HashSet<Point<i64>> = Default::default();

    for y in min.1..1000 {
        let steps = calculate_steps_required(y, &area);
        for x in 0..(1000 * min.0.signum()) {
            let velocity = Point::new(x, y);

            if let Some(_) = simulate(Point::new(0, 0), velocity, steps, &area) {
                unique_velocities.insert(velocity);
            }
        }
    }
    unique_velocities.len()
}

#[cfg(test)]
mod tests {
    use super::{calculate_new_velocity, simulate, star_one, star_two, Area, Point};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one((20, -10), (30, -5)), 45);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two((20, -10), (30, -5)), 112);
    }

    #[test]
    fn test_calculate_new_velocity() {
        let cases: &[(Point<i64>, Point<i64>)] = &[
            (Point::<i64>::new(0, 0), Point::<i64>::new(0, -1)),
            (Point::<i64>::new(1, 0), Point::<i64>::new(0, -1)),
            (Point::<i64>::new(-1, 0), Point::<i64>::new(0, -1)),
            (Point::<i64>::new(-5, 0), Point::<i64>::new(-4, -1)),
            (Point::<i64>::new(5, 0), Point::<i64>::new(4, -1)),
        ];

        for (velocity, expected_velocity) in cases {
            assert_eq!(calculate_new_velocity(*velocity), *expected_velocity);
        }
    }

    #[test]
    fn test_simulate() {
        let area = Area {
            min: Point::new(20, -10),
            max: Point::new(30, -5),
        };

        let result = simulate(Point::new(0, 0), Point::new(9, 0), 10, &area);

        assert!(result.is_some());
    }
}
