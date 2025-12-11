use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

type SplitterList = Vec<Vec<usize>>;

fn parse_content(content: &str) -> SplitterList {
    let splitter_list: SplitterList = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| usize::from(c == '^' || c == 'S'))
                .collect()
        })
        .filter(|line: &Vec<usize>| !line.iter().all(|x| *x == 0))
        .collect();

    splitter_list
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_final() {
        let splitter_list = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(
            splitter_list,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0]
            ]
        );
    }
}

fn fold(splitter_list: &SplitterList) -> usize {
    let mut beam_list: Vec<usize> = splitter_list[0].clone();
    // println!("BEAM:{beam_list:?}");

    for current_line in splitter_list.iter().skip(1) {
        // println!("CURR:{current_line:?}");
        let mut tmp_beam_list = beam_list.clone();
        beam_list
            .iter()
            .enumerate()
            .filter(|(_, beam_count)| **beam_count != 0)
            .for_each(|(y, beam_count)| {
                if current_line[y] == 1 {
                    tmp_beam_list[y] = 0;
                    tmp_beam_list[y - 1] += beam_count;
                    tmp_beam_list[y + 1] += beam_count;
                }
            });
        // println!("BEAM:{tmp_beam_list:?}");
        beam_list = tmp_beam_list;
    }

    beam_list.iter().sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_simple() {
        let map = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......",
        );
        assert_eq!(fold(&map), 4);
    }

    #[test]
    fn fold_simple2() {
        let map = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....",
        );
        assert_eq!(fold(&map), 8);
    }

    #[test]
    fn fold_final() {
        let map = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(fold(&map), 40);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let map = parse_content(&binding);
    println!("Result: {}", fold(&map));
}
