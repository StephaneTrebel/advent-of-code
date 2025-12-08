use std::{collections::HashMap, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(content: &str) -> Vec<(char, Vec<u64>)> {
    let iterator = content.lines();
    let line_count = iterator.clone().count();

    let mut number_line_list: HashMap<(usize, usize), char> = HashMap::new();
    let matrix = iterator.clone();
    matrix.enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            number_line_list.insert((x, y), c);
        });
    });

    let mut parsed_number_line_list: Vec<u64> = vec![];
    let mut number_map: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut y = 0;
    let mut z = 0;
    'outer: loop {
        // dbg!(&y);
        let mut tmp: Vec<char> = vec![];
        for x in 0..(line_count - 1) {
            let c = number_line_list.get(&(x, y));
            if let Some(&c) = c {
                tmp.push(c);
            } else {
                number_map.insert(z, parsed_number_line_list.clone());
                break 'outer;
            }
        }
        // dbg!(&tmp);
        let tmp2: String = tmp.iter().collect();
        if tmp2.trim() == "" {
            // dbg!("viiiiiiide");
            number_map.insert(z, parsed_number_line_list.clone());
            z += 1;
            parsed_number_line_list = vec![];
            y += 1;
            continue;
        }
        let parsed = tmp2.trim().parse::<u64>().unwrap();
        // dbg!(&parsed);
        parsed_number_line_list.push(parsed);
        y += 1;
    }

    // dbg!(&number_map);

    let operator_list: Vec<char> = iterator
        .clone()
        .skip(line_count - 1)
        .take(1)
        .next()
        .expect("Cannot read last line")
        .split_whitespace()
        .map(|operator| operator.parse::<char>().expect("Cannot parse character."))
        .collect();

    // dbg!(&operator_list);

    let result: Vec<(char, Vec<u64>)> = operator_list
        .iter()
        .enumerate()
        .map(|(idx_operator, operator)| {
            (
                operator.to_owned(),
                number_map.get(&idx_operator).unwrap().to_owned(),
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
105 3 
  2 42
+   * ",
        );
        assert_eq!(content, vec![('+', vec![1, 0, 52]), ('*', vec![34, 2])]);
    }

    #[test]
    fn parse_content_final() {
        let content = parse_content(
            "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(
            content,
            [
                ('*', vec![1, 24, 356]),
                ('+', vec![369, 248, 8]),
                ('*', vec![32, 581, 175]),
                ('+', vec![623, 431, 4])
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
2   421
+   *  ",
        );
        assert_eq!(fold(&content), 80);
    }

    #[test]
    fn fold_final() {
        let content = parse_content(
            "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(fold(&content), 3_263_827);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}

// 2509386693 is too low
