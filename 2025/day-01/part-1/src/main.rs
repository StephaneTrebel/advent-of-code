use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> Vec<String> {
    lines
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.to_owned())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
        );
        assert_eq!(
            content,
            vec![
                "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
            ]
        );
    }
}

fn fold(list: &[String]) -> isize {
    let mut position = 50;
    list.iter()
        .map(|movement| {
            let mut chars = movement.chars();
            let direction = chars.next().unwrap();
            let distance: isize = chars.collect::<String>().parse::<isize>().unwrap();
            if direction == 'L' {
                position -= distance;
            } else {
                position += distance;
            }
            if position % 100 == 0 { 1 } else { 0 }
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
L50
R50
R50
",
        );
        assert_eq!(fold(&content), 2);
    }

    #[test]
    fn fold_02() {
        let content = parse_content(
            "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
        );
        assert_eq!(fold(&content), 3);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}
