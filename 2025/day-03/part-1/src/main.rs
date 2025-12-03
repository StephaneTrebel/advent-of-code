use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> Vec<&str> {
    lines
        .split_whitespace()
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
987654321111111
811111111111119
234234234234278
818181911112111
",
        );
        assert_eq!(
            content,
            vec![
                "987654321111111",
                "811111111111119",
                "234234234234278",
                "818181911112111"
            ]
        );
    }
}

fn get_high_joltage(bank: &str) -> usize {
    let mut max1 = 0;
    let mut idx_max1 = 0;
    let mut max2 = 0;
    let mut idx_max2 = 0;
    bank.chars().enumerate().for_each(|(i, c)| {
        if i != bank.len() - 1 {
            let v = c.to_digit(10).unwrap() as usize;
            if v > max1 {
                max1 = v;
                idx_max1 = i;
            }
        }
    });
    // dbg!(&max1);
    // dbg!(&idx_max1);
    bank.chars()
        .enumerate()
        .skip(idx_max1 + 1)
        .for_each(|(i, c)| {
            let v = c.to_digit(10).unwrap() as usize;
            if v > max2 {
                max2 = v;
                idx_max2 = i;
            }
        });
    // dbg!(&max2);
    // dbg!(&idx_max2);
    max1 * 10 + max2
}

#[cfg(test)]
mod tests_get_high_joltage {
    use super::*;

    #[test]
    fn get_high_joltage_01() {
        let content = get_high_joltage("987654321111111");
        assert_eq!(content, 98);
    }
    #[test]
    fn get_high_joltage_02() {
        let content = get_high_joltage("811111111111119");
        assert_eq!(content, 89);
    }
    #[test]
    fn get_high_joltage_03() {
        let content = get_high_joltage("234234234234278");
        assert_eq!(content, 78);
    }
    #[test]
    fn get_high_joltage_04() {
        let content = get_high_joltage("818181911112111");
        assert_eq!(content, 92);
    }
}

fn fold(list: &[&str]) -> usize {
    list.iter().map(|bank| get_high_joltage(bank)).sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
987654321111111
811111111111119
234234234234278
818181911112111
",
        );
        assert_eq!(fold(&content), 357);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}
