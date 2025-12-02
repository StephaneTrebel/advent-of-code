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

fn fold(list: &[String], starting_position: i32) -> i32 {
    let mut position: i32 = starting_position;
    let mut sign = position.signum();
    list.iter()
        .map(|movement| {
            println!("------------------------------");
            dbg!(&movement);
            let mut chars = movement.chars();
            let direction = chars.next().unwrap();
            let distance: i32 = chars.collect::<String>().parse::<i32>().unwrap();
            let mut count = 0;

            // Apply rotation (Left or Right)
            if direction == 'L' {
                position -= distance;
            } else if direction == 'R' {
                position += distance;
            } else {
                panic!("Wat ?");
            }
            dbg!(&position);

            let rounds = position / 100;
            dbg!(&rounds);
            let modulo = position % 100;
            dbg!(&modulo);
            count += rounds.abs();

            if position == 0 {
                println!("position is 0 !");
                count += 1;
            } else if position.signum() != sign && sign != 0 {
                println!("position has changed sign !");
                count += 1;
            }

            if position.signum() == -1 && modulo != 0 {
                position = modulo + 100;
            } else if position > 100 {
                position = modulo;
            } else if modulo == 0 {
                position = 0;
            }
            dbg!(&position);

            sign = position.signum();

            dbg!(&count);
            count
        })
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_left_once_over_0() {
        let content = parse_content(
            "\
L68
",
        );
        assert_eq!(fold(&content, 50), 1);
    }

    #[test]
    fn fold_right_once_over_0() {
        let content = parse_content(
            "\
R68
",
        );
        assert_eq!(fold(&content, 50), 1);
    }

    #[test]
    fn fold_one_full_round_from_0() {
        let content = parse_content(
            "\
L100
",
        );
        assert_eq!(fold(&content, 0), 1);
    }

    #[test]
    fn fold_three_full_rounds_from_0() {
        let content = parse_content(
            "\
R300
",
        );
        assert_eq!(fold(&content, 0), 3);
    }

    #[test]
    fn fold_wat() {
        let content = parse_content(
            "\
L150
R50
",
        );
        assert_eq!(fold(&content, 50), 2);
    }

    #[test]
    fn fold_getting_to_0_with_more_than_one_round() {
        let content = parse_content(
            "\
L101
",
        );
        assert_eq!(fold(&content, 1), 2);
    }

    #[test]
    fn fold_ten_rounds_from_anywhere() {
        let content = parse_content(
            "\
R1000
",
        );
        assert_eq!(fold(&content, 17), 10);
    }

    #[test]
    fn fold_final() {
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
        assert_eq!(fold(&content, 50), 6);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content, 50));
}
