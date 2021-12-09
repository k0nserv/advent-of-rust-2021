use std::{collections::HashSet, rc::Rc};

fn neighbors(p: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);

    if p.0 > 0 {
        result.push((p.0 - 1, p.1));
    }

    if p.1 > 0 {
        result.push((p.0, p.1 - 1));
    }

    if p.0 < (max.0 - 1) {
        result.push((p.0 + 1, p.1));
    }

    if p.1 < (max.1 - 1) {
        result.push((p.0, p.1 + 1));
    }

    result
}

fn neighbors_recurse<F>(
    p: (usize, usize),
    max: (usize, usize),
    should_recurse: Rc<F>,
    result: &mut HashSet<(usize, usize)>,
) where
    F: Fn(&(usize, usize), &(usize, usize)) -> bool,
{
    let direct_neighbors = neighbors(p, max);

    for n in direct_neighbors {
        if should_recurse(&n, &p) && !result.contains(&n) {
            result.insert(n.clone());

            neighbors_recurse(n, max, should_recurse.clone(), result);
        }
    }
}

fn find_low_points(grid: &[Vec<u32>]) -> Vec<((usize, usize), u32)> {
    let grid_size = (grid[0].len(), grid.len());

    let grid_ref = &grid;

    grid.iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, p)| {
                let is_lowpoint = neighbors((x, y), grid_size)
                    .into_iter()
                    .all(|o| grid_ref[o.1][o.0] > *p);

                is_lowpoint.then(|| ((x, y), p + 1))
            })
        })
        .collect()
}

pub fn star_one(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 1)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    find_low_points(&grid).into_iter().map(|(_, p)| p).sum()
}

pub fn star_two(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 1)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let low_points = find_low_points(&grid);

    let grid_size = (grid[0].len(), grid.len());
    let grid_ref = &grid;

    let mut basins: Vec<HashSet<(usize, usize)>> = low_points
        .into_iter()
        .map(move |((x, y), _)| {
            let mut result = HashSet::with_capacity(4);
            result.insert((x, y));

            neighbors_recurse(
                (x, y),
                grid_size,
                Rc::new(|np: &(usize, usize), p: &(usize, usize)| {
                    let nv = grid_ref[np.1][np.0];

                    nv > grid_ref[p.1][p.0] && nv != 9
                }),
                &mut result,
            );

            result
        })
        .collect();

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    basins.into_iter().take(3).map(|b| b.len()).product()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 15);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 1134);
    }
}
