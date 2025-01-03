use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    ops::{Deref, DerefMut},
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Stones(
    HashMap<
        usize, // Stone ID
        usize, // Stone count
    >,
);

impl Deref for Stones {
    type Target = HashMap<usize, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stones {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<(usize, usize)>> for Stones {
    fn from(value: Vec<(usize, usize)>) -> Self {
        Self(HashMap::from_iter(value))
    }
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        Self(HashMap::from_iter(value.split_whitespace().map(|s| {
            (
                s.parse()
                    .unwrap_or_else(|_| panic!("{} should be a number", s)),
                1,
            )
        })))
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(stone, count)| format!("({}:{})", stone, count))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests_stones {
    use super::*;

    #[test]
    fn stones_from_simple() {
        let stones = Stones::from("10 20 30");
        pretty_assertions::assert_eq!(stones, Stones::from(vec![(10, 1), (20, 1), (30, 1)]));
    }
}

fn upsert(stones: &mut Stones, key: usize, closure: impl Fn(usize) -> usize) {
    match stones.get_mut(&key) {
        None => {
            stones.insert(key, closure(0));
        }
        Some(c) => {
            *c = closure(*c);
        }
    }
}

fn blink_once(stones: &mut Stones) {
    let iterator = stones.clone();
    iterator.iter().for_each(|(id, &count)| match id {
        0 => {
            upsert(stones, 1, |v| v + count);
            upsert(stones, 0, |v| v - count);
        }
        number => {
            let s = number.to_string();
            if s.chars().count() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);
                upsert(stones, left.parse().unwrap(), |v| v + count);
                upsert(stones, right.parse().unwrap(), |v| v + count);
            } else {
                upsert(stones, *id * 2024, |v| v + count);
            }
            upsert(stones, *id, |v| v - count);
        }
    });
}

#[cfg(test)]
mod tests_blink_once {
    use super::*;

    #[test]
    fn blink_once_01() {
        let mut stones = Stones::from("0 1 23 1000");
        blink_once(&mut stones);
        println!("Final Stones: {}", stones);
        dbg!(&stones, stones.get(&0), stones.get(&1));
        assert_eq!(stones.get(&0), Some(&1));
        assert_eq!(stones.get(&1), Some(&1));
        assert_eq!(stones.get(&10), Some(&1));
        assert_eq!(stones.get(&1000), Some(&0));
        assert_eq!(stones.get(&2), Some(&1));
        assert_eq!(stones.get(&2024), Some(&1));
        assert_eq!(stones.get(&23), Some(&0));
        assert_eq!(stones.get(&3), Some(&1));
    }
}

fn blink_n_times(stones: &mut Stones, n: usize) {
    for _ in 0..n {
        blink_once(stones);
    }
}

#[cfg(test)]
mod tests_blink_n_times {
    use super::*;

    #[test]
    fn blink_n_times_01() {
        let mut stones = Stones::from("125 17");
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 3);
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 4);
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 5);
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 9);
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 13);
        blink_n_times(&mut stones, 1);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 22);
        blink_n_times(&mut stones, 19);
        assert_eq!(stones.iter().map(|(_, v)| *v).sum::<usize>(), 55312);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let mut stones = Stones::from(get_file_content("assets/input").as_str());
    blink_n_times(&mut stones, 75);
    println!("Result: {}", stones.iter().map(|(_, v)| *v).sum::<usize>());
}
