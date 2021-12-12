use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    str::FromStr,
};

type ID = String;

#[derive(Debug)]
struct Cave {
    id: ID,
    is_small: bool,
    connections: HashSet<ID>,
}

impl Cave {
    fn new(id: ID, is_small: bool) -> Self {
        Self {
            id,
            is_small,
            connections: Default::default(),
        }
    }

    fn add_connection(&mut self, to: ID) {
        self.connections.insert(to);
    }
}

#[derive(Debug)]
struct System {
    start: ID,
    end: ID,
    caves: HashMap<ID, Cave>,
}

impl System {
    fn find_cave(&self, id: &ID) -> &Cave {
        self.caves.get(id).unwrap()
    }
}

impl FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut caves = HashMap::default();

        let clean_lines = s.lines().map(str::trim).filter(|l| l.len() > 0);

        fn add_cave(caves: &mut HashMap<ID, Cave>, id: &str, other_id: &str) {
            let cave = caves
                .entry(id.into())
                .or_insert_with(|| Cave::new(id.into(), id.chars().all(char::is_lowercase)));

            cave.add_connection(other_id.into());
        }

        for line in clean_lines {
            let mut parts = line.split("-");

            match (parts.next().map(str::trim), parts.next().map(str::trim)) {
                (Some(c1_id), Some(c2_id)) => {
                    add_cave(&mut caves, c1_id, c2_id);
                    add_cave(&mut caves, c2_id, c1_id);

                    if c1_id == "start" {
                        start = Some(c1_id.into());
                    }

                    if c2_id == "start" {
                        start = Some(c2_id.into());
                    }

                    if c1_id == "end" {
                        end = Some(c1_id.into());
                    }

                    if c2_id == "end" {
                        end = Some(c2_id.into());
                    }
                }
                _ => {
                    return Err(format!("Invalid cave system:\n{}", s));
                }
            };
        }

        Ok(System {
            start: start.unwrap(),
            end: end.unwrap(),
            caves,
        })
    }
}

#[derive(Default, Debug)]
struct State {
    path: LinkedList<ID>,
    visited_small_caves: HashMap<ID, usize>,
}

impl State {
    fn new(initial: ID) -> Self {
        let mut list = LinkedList::new();
        list.push_front(initial);

        Self {
            path: list,
            visited_small_caves: Default::default(),
        }
    }
}

fn explore<F>(system: &System, from: ID, mut should_continue: F) -> Vec<Vec<ID>>
where
    F: FnMut(&Cave, &State) -> bool,
{
    let mut stack = VecDeque::<State>::new();
    {
        let mut initial_state = State::new(from.clone());
        if from.chars().all(char::is_lowercase) {
            initial_state.visited_small_caves.insert(from, 1);
        }
        stack.push_back(initial_state);
    }

    let mut end_states: Vec<State> = vec![];

    while let Some(state) = stack.pop_back() {
        let current = state.path.back().unwrap();
        let cave = system.find_cave(current);

        for next_id in cave.connections.iter() {
            let next_cave = system.find_cave(next_id);

            if !should_continue(&next_cave, &state) {
                continue;
            }

            let path = {
                let mut p = state.path.clone();
                p.push_back(next_id.clone());

                p
            };
            let visited_small_caves = {
                let mut s = state.visited_small_caves.clone();

                if next_cave.is_small {
                    *s.entry(next_id.into()).or_default() += 1;
                }

                s
            };
            let new_state = State {
                path,
                visited_small_caves,
            };

            if next_id == &system.end {
                end_states.push(new_state);
            } else {
                stack.push_back(new_state);
            }
        }
    }

    end_states
        .into_iter()
        .map(|s| s.path.into_iter().collect())
        .collect()
}

pub fn star_one(input: &str) -> usize {
    let system = System::from_str(input).expect("Failed to parse cave system");

    let paths = explore(&system, system.start.clone(), |cave, state| {
        state
            .visited_small_caves
            .get(&cave.id)
            .map(|c| c == &0)
            .unwrap_or(true)
    });

    paths.len()
}

pub fn star_two(input: &str) -> usize {
    let system = System::from_str(input).expect("Failed to parse cave system");

    let paths = explore(&system, system.start.clone(), |cave, state| {
        // if cave.id == system.start && state.path.len() == 1 {
        //     return true;
        // }

        if cave.id == system.start {
            return false;
        }

        let visited_twice =
            state
                .visited_small_caves
                .iter()
                .find_map(|(id, c)| if c >= &2 { Some(id) } else { None });

        state
            .visited_small_caves
            .get(&cave.id)
            .map(|c| (visited_twice.is_none() && c < &2) || (visited_twice.is_some() && c == &0))
            .unwrap_or(true)
    });

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    const INPUT_MEDIUM: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    const INPUT_LARGE: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn test_star_one() {
        for (input, result) in &[(INPUT, 10), (INPUT_MEDIUM, 19), (INPUT_LARGE, 226)] {
            assert_eq!(
                star_one(input),
                *result,
                "Wrong result returned for: \n{}",
                input
            );
        }
    }

    #[test]
    fn test_star_two() {
        for (input, result) in &[(INPUT, 36), (INPUT_MEDIUM, 103), (INPUT_LARGE, 3509)] {
            assert_eq!(
                star_two(input),
                *result,
                "Wrong result returned for: \n{}",
                input
            );
        }
    }
}
