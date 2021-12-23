use std::collections::{BinaryHeap, HashMap};
use std::ops::Index;

type Grid = Vec<Vec<u8>>;

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

fn neighbors(p: Point<usize>, max: Point<usize>) -> Vec<Point<usize>> {
    let mut result = Vec::with_capacity(4);

    if p.x > 0 {
        result.push(Point::new(p.x - 1, p.y));
    }

    if p.y > 0 {
        result.push(Point::new(p.x, p.y - 1));
    }

    if p.x < (max.x - 1) {
        result.push(Point::new(p.x + 1, p.y));
    }

    if p.y < (max.y - 1) {
        result.push(Point::new(p.x, p.y + 1));
    }

    result
}

#[derive(Debug)]
struct HeapEntry<T> {
    location: Point<usize>,
    value: T,
}

impl<T: Ord> HeapEntry<T> {
    fn new(location: Point<usize>, value: T) -> Self {
        Self { location, value }
    }
}

impl<T: Ord> Ord for HeapEntry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}
impl<T: Ord> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialEq> PartialEq for HeapEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.location.eq(&other.location) && self.value.eq(&other.value)
    }
}
impl<T: Eq> Eq for HeapEntry<T> {}

impl<T> Index<Point<usize>> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl<T> Index<&Point<usize>> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: &Point<usize>) -> &Self::Output {
        &self[index.y][index.x]
    }
}

fn reconstruct_path(
    came_from: HashMap<Point<usize>, Point<usize>>,
    from: Point<usize>,
) -> Vec<Point<usize>> {
    let mut path = vec![from];
    let mut current = &from;

    while let Some(next) = came_from.get(current) {
        path.push(next);
        current = next;
    }

    path.reverse();
    path
}

fn djikstra(grid: &Grid, from: Point<usize>, to: Point<usize>) -> Option<Vec<Point<usize>>> {
    let max = Point::new(grid[0].len(), grid.len());

    let mut open = BinaryHeap::new();
    for x in 0..max.x {
        for y in 0..max.y {
            let p = Point::new(x, y);

            if p != from {
                open.push(HeapEntry::new(Point::new(x, y), usize::MAX));
            }
        }
    }
    open.push(HeapEntry::new(from, grid[from].into()));

    let mut came_from: HashMap<Point<usize>, Point<usize>> = Default::default();

    let mut distance: HashMap<Point<usize>, usize> = Default::default();
    distance.insert(from, grid[from].into());

    while !open.is_empty() {
        let current = open.pop().unwrap();

        if current.location == to {
            return Some(reconstruct_path(came_from, to));
        }

        for neighbor in neighbors(current.location, max) {
            let alt = distance
                .get(&current.location)
                .map(|d| d + grid[neighbor] as usize)
                .unwrap_or(usize::MAX);

            if alt < *distance.get(&neighbor).unwrap_or(&usize::MAX) {
                distance.insert(neighbor, alt);
                came_from.insert(neighbor, current.location);
                open.push(HeapEntry::new(neighbor, alt));
            }
        }
    }

    None
}

pub fn star_one(input: &str) -> usize {
    let grid: Grid = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let goal = Point::new(grid[0].len() - 1, grid.len() - 1);
    let path = djikstra(&grid, Point::<usize>::ORIGIN, goal).unwrap();

    path.into_iter().skip(1).map(|p| grid[p] as usize).sum()
}

pub fn star_two(input: &str) -> usize {
    let grid: Grid = {
        let og_grid: Grid = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let og_size = Point::new(og_grid[0].len(), og_grid.len());
        let grid_size = Point::new(og_size.x * 5, og_size.y * 5);

        let mut grid: Grid = vec![];
        for _ in 0..grid_size.y {
            let row = Vec::with_capacity(grid_size.x * 5);
            grid.push(row);
        }

        for y in 0..grid_size.y {
            for x in 0..grid_size.x {
                if x < og_size.x && y < og_size.y {
                    grid[y].push(og_grid[y][x]);
                } else {
                    let x0 = if x >= og_size.x && y < og_size.y {
                        x - og_size.x
                    } else {
                        x
                    };
                    let y0 = if y >= og_size.y { y - og_size.y } else { y };
                    let value = ((grid[y0][x0] + 1) % 10).max(1);

                    grid[y].push(value);
                }
            }
        }

        grid
    };

    let goal = Point::new(grid[0].len() - 1, grid.len() - 1);
    let path = djikstra(&grid, Point::<usize>::ORIGIN, goal).unwrap();

    path.into_iter().skip(1).map(|p| grid[p] as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 40);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 315);
    }
}
