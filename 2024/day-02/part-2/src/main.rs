use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Content {
    list: Vec<Vec<usize>>,
}

fn parse_content(lines: &str) -> Content {
    Content {
        list: lines
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .filter(|l| !l.is_empty())
            .collect::<Vec<Vec<usize>>>(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(
            content,
            Content {
                list: vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ]
            }
        );
    }
}

fn is_monotonic_and_bounded(list: &[usize], can_try_again: bool) -> bool {
    // dbg!(list, can_try_again);

    // Trivial case, no comparison needed
    if list.len() < 2 {
        return true;
    }

    let mut need_new_attempt = false;

    // We're going to test if direction is valid (|direction| < 3) so both its
    // dependent (index 0 and 1) can be levels that could be remove to make the level list fine
    let mut suspect_1 = 0;
    let mut suspect_2 = 1;
    // Possibly needed for Signum check
    let mut suspect_3: Option<usize> = None;
    let direction: isize = list[suspect_1] as isize - list[suspect_2] as isize;

    if direction.unsigned_abs() > 3 || direction.signum() == 0 {
        need_new_attempt = true;
    } else {
        for index in 1..list.len() - 1 {
            let temp: isize = list[index] as isize - list[index + 1] as isize;
            let mut ok = true;
            if temp.unsigned_abs() > 3 || temp.signum() != direction.signum() {
                ok = false;
            }
            if !ok {
                need_new_attempt = true;
                suspect_1 = index;
                suspect_2 = index + 1;
                // In case of a Signum check, maybe it's the first element that is faulty
                // Consider [10,11,10,9,8,7]. In this case it's the first element that is an issue
                suspect_3 = Some(index - 1);
                // dbg!(suspect_1, suspect_2, suspect_3);
                break;
            }
        }
    }

    // dbg!(need_new_attempt);
    if need_new_attempt {
        if can_try_again {
            // dbg!(can_try_again);
            let mut list_without_faulty_level_1 = Vec::from(list);
            list_without_faulty_level_1.remove(suspect_1);
            // dbg!(&list_without_faulty_level_1);
            let result_without_suspect_1 =
                is_monotonic_and_bounded(&list_without_faulty_level_1, false);

            let mut list_without_faulty_level_2 = Vec::from(list);
            list_without_faulty_level_2.remove(suspect_2);
            // dbg!(&list_without_faulty_level_2);
            let result_without_suspect_2 =
                is_monotonic_and_bounded(&list_without_faulty_level_2, false);

            let mut result_without_suspect_3 = false;
            if let Some(suspect) = suspect_3 {
                if list.get(suspect).is_some() {
                    let mut list_without_faulty_level_3 = Vec::from(list);
                    list_without_faulty_level_3.remove(suspect);
                    result_without_suspect_3 =
                        is_monotonic_and_bounded(&list_without_faulty_level_3, false);
                }
            }

            if result_without_suspect_3 {
                dbg!(list);
            }

            result_without_suspect_1 || result_without_suspect_2 || result_without_suspect_3
        } else {
            false
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests_is_monotonic_and_bounded {
    use super::*;

    #[test]
    fn is_monotonic_and_bounded_01() {
        assert!(is_monotonic_and_bounded(&[7, 6, 4, 2, 1], false));
        assert!(!is_monotonic_and_bounded(&[1, 3, 2, 4, 5], false));
        assert!(is_monotonic_and_bounded(&[1, 3, 2, 4, 5], true));
        assert!(!is_monotonic_and_bounded(
            &[67, 73, 75, 77, 83, 85, 90],
            false
        ));
        assert!(!is_monotonic_and_bounded(
            &[67, 73, 75, 77, 83, 85, 90],
            true
        ));
        assert!(!is_monotonic_and_bounded(
            &[29, 28, 25, 21, 20, 18, 16, 17],
            true
        ));
        assert!(is_monotonic_and_bounded(&[37, 38, 37, 34, 31], true));
    }
}

fn fold_delta(content: &Content) -> usize {
    content
        .list
        .iter()
        .map(|levels| {
            let result = is_monotonic_and_bounded(levels, true);
            println!("{:?} is {}", levels, {
                if result {
                    "OK"
                } else {
                    "KO"
                }
            });
            result
        })
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests_fold_delta {
    use super::*;

    #[test]
    fn fold_delta_01() {
        let content = parse_content(
            "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(fold_delta(&content), 4);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold_delta(content));
}
