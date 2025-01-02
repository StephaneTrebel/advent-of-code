use std::{fmt::Display, fs, slice::Iter};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Stones(Vec<usize>);

impl Stones {
    fn iter(&self) -> Iter<'_, usize> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<usize>> for Stones {
    fn from(value: Vec<usize>) -> Self {
        Self(value)
    }
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|s| {
                    s.parse()
                        .unwrap_or_else(|_| panic!("{} should be a number", s))
                })
                .collect(),
        )
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|stone| stone.to_string())
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
        pretty_assertions::assert_eq!(stones, Stones::from(vec![10, 20, 30]));
        pretty_assertions::assert_eq!(format!("{}", stones), "10 20 30");
    }
}

fn blink_once(stones: &Stones) -> Stones {
    Stones(
        stones
            .iter()
            .flat_map(|stone| match stone {
                0 => vec![1],
                number if number.to_string().chars().count() % 2 == 0 => {
                    let s = number.to_string();
                    let (left, right) = s.split_at(s.len() / 2);
                    vec![left.parse().unwrap(), right.parse().unwrap()]
                }
                x => vec![x * 2024],
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests_blink_once {
    use super::*;

    #[test]
    fn blink_once_01() {
        let stones = Stones::from("0 1 23 1000");
        let new_stones = blink_once(&stones);
        pretty_assertions::assert_eq!(new_stones, Stones::from(vec![1, 2024, 2, 3, 10, 0]));
        pretty_assertions::assert_eq!(format!("{}", new_stones), "1 2024 2 3 10 0");
    }
}

fn blink_n_times(stones: &Stones, n: usize) -> Stones {
    let mut temp = stones.clone();
    for _ in 0..n {
        temp = blink_once(&temp);
    }
    temp
}

#[cfg(test)]
mod tests_blink_n_times {
    use super::*;

    #[test]
    fn blink_n_times_01() {
        let mut stones = Stones::from("125 17");
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(stones, Stones::from(vec![253000, 1, 7]));
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(stones, Stones::from(vec![253, 0, 2024, 14168]));
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(stones, Stones::from(vec![512072, 1, 20, 24, 28676032]));
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(
            stones,
            Stones::from(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])
        );
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(
            stones,
            Stones::from(vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
            ])
        );
        stones = blink_n_times(&stones, 1);
        pretty_assertions::assert_eq!(
            stones,
            Stones::from(vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ])
        );
        stones = blink_n_times(&stones, 19);
        assert_eq!(stones.len(), 55312);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let stones = Stones::from(get_file_content("assets/input").as_str());

    println!("Result: {}", blink_n_times(&stones, 25).len());
}
