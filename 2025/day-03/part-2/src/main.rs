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

fn get_high_joltage(bank: &str, battery_size: usize) -> usize {
    let mut digits: Vec<usize> = vec![];
    let mut idx_digits: Vec<usize> = vec![];

    for i in (1..=battery_size).rev() {
        let skip = {
            if i == battery_size {
                0
            } else {
                match idx_digits.get(battery_size - i - 1) {
                    Some(index) => index + 1,
                    None => 0,
                }
            }
        };
        bank.chars().enumerate().skip(skip).for_each(|(idx, c)| {
            if idx <= (bank.len() - i) {
                let v = c.to_digit(10).unwrap() as usize;
                if digits.get(battery_size - i).is_none() {
                    digits.push(v);
                    idx_digits.push(idx);
                } else if v > digits[battery_size - i] {
                    digits[battery_size - i] = v;
                    idx_digits[battery_size - i] = idx;
                }
            }
        });
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| 10_usize.pow(u32::try_from(i).unwrap()) * d)
        .sum()
}

#[cfg(test)]
mod tests_get_high_joltage {
    use super::*;

    #[test]
    fn get_high_joltage_01() {
        let content = get_high_joltage("987654321111111", 12);
        assert_eq!(content, 987_654_321_111);
    }
    #[test]
    fn get_high_joltage_02() {
        let content = get_high_joltage("811111111111119", 12);
        assert_eq!(content, 811_111_111_119);
    }
    #[test]
    fn get_high_joltage_03() {
        let content = get_high_joltage("234234234234278", 12);
        assert_eq!(content, 434_234_234_278);
    }
    #[test]
    fn get_high_joltage_04() {
        let content = get_high_joltage("818181911112111", 12);
        assert_eq!(content, 888_911_112_111);
    }
}

fn fold(list: &[&str]) -> usize {
    list.iter().map(|bank| get_high_joltage(bank, 12)).sum()
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
        assert_eq!(fold(&content), 3_121_910_778_619);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}

// 173848577117276
