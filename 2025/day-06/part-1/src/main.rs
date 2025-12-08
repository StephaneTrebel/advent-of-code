use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(content: &str) -> Vec<(char, Vec<u64>)> {
    let iterator = content.lines();

    let line_count = iterator.clone().count();

    let number_line_list: Vec<Vec<u64>> = iterator
        .clone()
        .take(line_count - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|number| {
                    number.parse::<u64>().expect("Cannot parse number.")
                })
                .collect()
        })
        .collect();

    let operator_list: Vec<char> = iterator
        .clone()
        .skip(line_count - 1)
        .take(1)
        .next()
        .expect("Cannot read last line")
        .split_whitespace()
        .map(|operator| operator.parse::<char>().expect("Cannot parse character."))
        .collect();
    dbg!(&operator_list.len());

    let result: Vec<(char, Vec<u64>)> = operator_list
        .iter()
        .enumerate()
        .map(|(idx_operator, operator)| {
            (
                operator.to_owned(),
                number_line_list
                    .iter()
                    .map(|number_list| number_list[idx_operator])
                    .collect(),
            )
        })
        .collect();

    result
}

#[cfg(test)]
mod tests_parse_content {
    use std::vec;

    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
100 3
2 400
+ *",
        );
        assert_eq!(content, vec![('+', vec![100, 2]), ('*', vec![3, 400])]);
    }

    #[test]
    fn parse_content_final() {
        let content = parse_content(
            "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +",
        );
        assert_eq!(
            content,
            [
                ('*', vec![123, 45, 6]),
                ('+', vec![328, 64, 98]),
                ('*', vec![51, 387, 215]),
                ('+', vec![64, 23, 314])
            ]
        );
    }
}

fn fold(worksheet: &[(char, Vec<u64>)]) -> u64 {
    worksheet
        .iter()
        .map(|(operator, number_list)| match operator {
            '+' => number_list.iter().sum::<u64>(),
            '*' => number_list.iter().product::<u64>(),
            _ => panic!("WHAT THE ACTUAL FUCK !?"),
        })
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_simple() {
        let content = parse_content(
            "\
100 3
2 400
+ *",
        );
        assert_eq!(fold(&content), 1302);
    }

    #[test]
    fn fold_final() {
        let content = parse_content(
            "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
",
        );
        assert_eq!(fold(&content), 4_277_556);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}

// 2509386693 is too low
