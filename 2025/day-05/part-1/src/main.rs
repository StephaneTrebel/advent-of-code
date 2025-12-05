use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut split = lines.split('\n');
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    for line in split.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut inner_split = line.split('-');
        let start = inner_split
            .next()
            .expect("Missing start value in range")
            .parse::<u64>()
            .expect("Cannot parse start value");
        let end = inner_split
            .next()
            .expect("Missing end value in range")
            .parse::<u64>()
            .expect("Cannot parse end value");
        ranges.push((start, end));
    }
    println!("ranges: {ranges:?}");

    for line in split {
        if line.is_empty() {
            break;
        }
        ingredients.push(line.parse::<u64>().expect("Cannot parse ingredient value"));
    }
    println!("ingredients: {ingredients:?}");

    (ranges, ingredients)
}

#[cfg(test)]
mod tests_parse_content {
    use std::vec;

    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
1-3
4-5

6
7
",
        );
        assert_eq!(content, (vec![(1, 3), (4, 5)], vec![6, 7]));
    }

    #[test]
    fn parse_content_final() {
        let content = parse_content(
            "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
        );
        assert_eq!(
            content,
            (
                vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
    }
}

fn fold((ranges, ingredients): &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    let all_ingredients_count = ingredients.len();
    let spoiled_ingredients_count = ingredients
        .iter()
        .filter(|&&ingredient| {
            ranges
                .iter()
                .all(|&range| ingredient < range.0 || ingredient > range.1)
        })
        .count();

    all_ingredients_count - spoiled_ingredients_count
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_final() {
        let content = parse_content(
            "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
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

// 339 is too low
