use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> Vec<String> {
    lines
        .split(',')
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
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(
            content,
            vec![
                "11-22",
                "95-115",
                "998-1012",
                "1188511880-1188511890",
                "222220-222224",
                "1698522-1698528",
                "446443-446449",
                "38593856-38593862",
                "565653-565659",
                "824824821-824824827",
                "2121212118-2121212124"
            ]
        );
    }
}

fn get_invalid_ids(first_id: usize, last_id: usize) -> Vec<usize> {
    let mut invalid_ids: Vec<usize> = vec![];

    for i in first_id..=last_id {
        let i_str = i.to_string();
        let len_i = i_str.len();

        for step in (1..=len_i).rev() {
            if len_i.is_multiple_of(step) {
                let values: Vec<&[u8]> = i_str.as_bytes().chunks(step).collect();
                if values.len() <= 1 {
                    continue;
                }
                if values.iter().all(|v| *v == values[0]) {
                    invalid_ids.push(i);
                    break;
                }
            }
        }
    }
    invalid_ids
}

#[cfg(test)]
mod tests_get_invalid_ids {
    use super::*;

    #[test]
    fn get_invalid_ids_01() {
        assert_eq!(get_invalid_ids(11, 22), vec![11, 22]);
    }
    #[test]
    fn get_invalid_ids_02() {
        assert_eq!(get_invalid_ids(110, 110), vec![]);
    }
    #[test]
    fn get_invalid_ids_03() {
        assert_eq!(get_invalid_ids(998, 1012), vec![999, 1010]);
    }
    #[test]
    fn get_invalid_ids_04() {
        assert_eq!(
            get_invalid_ids(1_188_511_880, 1_188_511_890),
            vec![1_188_511_885]
        );
    }
    #[test]
    fn get_invalid_ids_05() {
        assert_eq!(get_invalid_ids(222_220, 222_224), vec![222_222]);
    }
    #[test]
    fn get_invalid_ids_06() {
        assert_eq!(get_invalid_ids(1_698_522, 1_698_528), vec![]);
    }
    #[test]
    fn get_invalid_ids_07() {
        assert_eq!(get_invalid_ids(446_443, 446_449), vec![446_446]);
    }
    #[test]
    fn get_invalid_ids_08() {
        assert_eq!(get_invalid_ids(38_593_856, 38_593_862), vec![38_593_859]);
    }
    #[test]
    fn get_invalid_ids_09() {
        assert_eq!(get_invalid_ids(565_653, 565_659), vec![565_656]);
    }
    #[test]
    fn get_invalid_ids_10() {
        assert_eq!(get_invalid_ids(824_824_821, 824_824_827), vec![824_824_824]);
    }
    #[test]
    fn get_invalid_ids_11() {
        assert_eq!(
            get_invalid_ids(2_121_212_118, 2_121_212_124),
            vec![2_121_212_121]
        );
    }
}

fn fold(list: &[String]) -> usize {
    list.iter()
        .map(|range| {
            let mut split = range.split('-');
            let first_id = split.next().unwrap().parse::<usize>().unwrap();
            let last_id = split.next().unwrap().trim_end().parse::<usize>().unwrap();
            get_invalid_ids(first_id, last_id).iter().sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content("11-22");
        assert_eq!(fold(&content), 33);
    }
    #[test]
    fn fold_02() {
        let content = parse_content(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(fold(&content), 4_174_379_265);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}
