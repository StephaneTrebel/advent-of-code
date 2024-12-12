use std::{collections::VecDeque, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Test = (usize, VecDeque<usize>);
type Tests = Vec<Test>;
#[derive(Debug, PartialEq, Clone)]
struct Content {
    tests: Tests,
}

fn parse_content(input: &str) -> Content {
    Content {
        tests: input
            .split("\n")
            .filter_map(|line| {
                if !line.is_empty() {
                    let mut split = line.split(":");
                    Some((
                        split
                            .next()
                            .expect("Test should have an expected result")
                            .parse()
                            .expect("Test result should be parseable"),
                        split
                            .next()
                            .expect("Test should have operands")
                            .split(" ")
                            .map(|s| s.parse().expect("Operand should be parseable"))
                            .collect(),
                    ))
                } else {
                    None
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
190:10 19
3267:81 40 27
83:17 5
156:15 6
7290:6 8 6 15
161011:16 10 13
192:17 8 14
21037:9 7 18 13
292:11 6 16 20
",
        );
        assert_eq!(
            content,
            Content {
                tests: vec![
                    (190, VecDeque::from([10, 19])),
                    (3267, VecDeque::from([81, 40, 27])),
                    (83, VecDeque::from([17, 5])),
                    (156, VecDeque::from([15, 6])),
                    (7290, VecDeque::from([6, 8, 6, 15])),
                    (161011, VecDeque::from([16, 10, 13])),
                    (192, VecDeque::from([17, 8, 14])),
                    (21037, VecDeque::from([9, 7, 18, 13])),
                    (292, VecDeque::from([11, 6, 16, 20]))
                ]
            }
        );
    }
}

/// [a,b] => a * b OR a + b
/// [a,b,c] => a * b + c OR a + b * c OR a + b + c OR a * b * c
fn is_computable((expected, operands): Test, acc: usize) -> bool {
    // dbg!(&expected, &operands);
    if operands.is_empty() {
        return acc == expected;
    }
    let mut remains = operands.clone();
    let operand = remains.pop_front().expect("Operands should not be empty");
    is_computable((expected, remains.clone()), acc * operand)
        || is_computable((expected, remains.clone()), acc + operand)
}

#[cfg(test)]
mod tests_is_computable {
    use super::*;

    #[test]
    fn is_computable_01() {
        assert!(is_computable((190, VecDeque::from([10, 19])), 0));
        assert!(is_computable((3267, VecDeque::from([81, 40, 27])), 0));
        assert!(!is_computable((83, VecDeque::from([17, 5])), 0));
        assert!(!is_computable((156, VecDeque::from([15, 6])), 0));
        assert!(!is_computable((7290, VecDeque::from([6, 8, 6, 15])), 0));
        assert!(!is_computable((161011, VecDeque::from([16, 10, 13])), 0));
        assert!(!is_computable((192, VecDeque::from([17, 8, 14])), 0));
        assert!(!is_computable((21037, VecDeque::from([9, 7, 18, 13])), 0));
        assert!(is_computable((292, VecDeque::from([11, 6, 16, 20])), 0));
    }
}

fn fold(tests: &Tests) -> usize {
    tests
        .iter()
        .map(|test| {
            if is_computable(test.clone(), 0) {
                test.0
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
190:10 19
3267:81 40 27
83:17 5
156:15 6
7290:6 8 6 15
161011:16 10 13
192:17 8 14
21037:9 7 18 13
292:11 6 16 20
",
        );
        assert_eq!(fold(&content.tests), 3749);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &mut parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(&content.tests));
}
