use regex::Regex;
use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Content {
    list: String,
}

fn parse_content(lines: &str) -> Content {
    Content {
        list: lines.to_owned(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content("toto");
        assert_eq!(
            content,
            Content {
                list: "toto".to_owned()
            }
        );
    }
}

fn clean_up(list: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let results: Vec<_> = re.find_iter(list).map(|m| m.as_str()).collect();
    results
}

#[cfg(test)]
mod tests_clean_up {
    use super::*;

    #[test]
    fn clean_up_01() {
        assert_eq!(
            clean_up("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)",]
        )
    }
}

fn execute_operation(operation: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let results: Vec<_> = re.find_iter(operation).map(|m| m.as_str()).collect();
    results
        .iter()
        .map(|c| c.parse().unwrap())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

#[cfg(test)]
mod tests_execute_operation {
    use super::*;

    #[test]
    fn execute_operation_01() {
        assert_eq!(execute_operation("mul(2,4)"), 8);
        assert_eq!(execute_operation("mul(5,5)"), 25);
    }
}

fn fold_executions(content: &Content) -> usize {
    clean_up(&content.list)
        .iter()
        .map(|operation| execute_operation(operation))
        .sum()
}

#[cfg(test)]
mod tests_fold_executions {
    use super::*;

    #[test]
    fn fold_executions_01() {
        let content = parse_content(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(fold_executions(&content), 161);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold_executions(content));
}
