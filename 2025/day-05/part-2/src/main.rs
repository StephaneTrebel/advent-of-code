use std::{collections::HashSet, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> Vec<(u64, u64)> {
    let mut split = lines.split('\n');
    let mut ranges: Vec<(u64, u64)> = vec![];

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

    ranges
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
",
        );
        assert_eq!(content, vec![(1, 3), (4, 5)]);
    }

    #[test]
    fn parse_content_final() {
        let content = parse_content(
            "\
3-5
10-14
16-20
12-18
",
        );
        assert_eq!(content, vec![(3, 5), (10, 14), (16, 20), (12, 18)],);
    }
}

fn fold(ranges: &mut Vec<(u64, u64)>) -> u64 {
    let mut to_delete: HashSet<usize> = HashSet::new();
    let mut temp: Vec<(u64, u64)> = vec![];

    'outer: loop {
        for &index in &to_delete {
            ranges.remove(index);
        }
        to_delete = HashSet::new();

        temp.clone_from(ranges);
        for (index_range, range) in ranges.iter_mut().enumerate() {
            for (index_other_range, other_range) in temp.iter().enumerate() {
                if range.1 < other_range.0 || range.0 > other_range.1 {
                    continue;
                }
                if other_range.0 == range.0
                    && range.1 == other_range.1
                    && index_range != index_other_range
                {
                    to_delete.insert(index_other_range);
                    continue 'outer;
                }
                if other_range.0 < range.0 || range.1 < other_range.1 {
                    if other_range.0 < range.0 {
                        range.0 = other_range.0;
                    }
                    if range.1 < other_range.1 {
                        range.1 = other_range.1;
                    }
                    to_delete.insert(index_other_range);
                    continue 'outer;
                }
            }
        }

        break;
    }

    ranges.iter().map(|range| range.1 - range.0 + 1).sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_disjointed() {
        let content = &mut parse_content(
            "\
3-5
10-14
",
        );
        assert_eq!(fold(content), 8);
    }

    #[test]
    fn fold_extend_lower_bound() {
        let content = &mut parse_content(
            "\
16-20
12-18
",
        );
        assert_eq!(fold(content), 9);
    }

    #[test]
    fn fold_final() {
        let content = &mut parse_content(
            "\
3-5
10-14
16-20
12-18
",
        );
        assert_eq!(fold(content), 14);
    }

    #[test]
    fn fold_moar01() {
        let content = &mut parse_content(
            "\
100-101
50-70
10-10
99-100
100-101
200-300
200-300
250-251
98-99
100-100
100-101
1-101
",
        );
        assert_eq!(fold(content), 202);
    }

    #[test]
    fn fold_moar02() {
        let content = &mut parse_content(
            "\
200-300
100-101
1-1
2-2
3-3
1-3
1-3
2-2
50-70
10-10
98-99
99-99
99-99
99-100
1-1
2-1
100-100
100-100
100-101
200-300
201-300
202-300
250-251
98-99
100-100
100-101
1-101
",
        );
        assert_eq!(fold(content), 202);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &mut parse_content(&binding);
    println!("Result: {}", fold(content));
}
