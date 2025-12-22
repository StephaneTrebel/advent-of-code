use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coords = (u64, u64);

fn parse_content(content: &str) -> Vec<Coords> {
    content
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_sample() {
        let coord_list = parse_content(
            "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(
            coord_list,
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3)
            ]
        );
    }
}

#[allow(clippy::too_many_lines)]
/// X.............
/// .X.....#...#..
/// ..............
/// ..#....#......
/// ..............
/// ..#......#....
/// ..............
/// .........#.#..
/// ..............
fn compute_rectangle_area((xa, ya): Coords, (xb, yb): Coords) -> u64 {
    (xb.abs_diff(xa) + 1) * (yb.abs_diff(ya) + 1)
}

#[cfg(test)]
mod tests_compute_rectangle_area {
    use super::*;

    #[test]
    fn compute_rectangle_area_01() {
        assert_eq!(compute_rectangle_area((1, 1), (2, 2)), 4);
    }

    #[test]
    fn compute_rectangle_area_02() {
        assert_eq!(compute_rectangle_area((0, 0), (1, 1)), 4);
    }

    #[test]
    fn compute_rectangle_area_03() {
        assert_eq!(compute_rectangle_area((1, 1), (0, 0)), 4);
    }

    #[test]
    fn compute_rectangle_area_04() {
        assert_eq!(compute_rectangle_area((2, 5), (9, 7)), 24);
    }

    #[test]
    fn compute_rectangle_area_05() {
        assert_eq!(compute_rectangle_area((11, 7), (7, 1)), 35);
    }

    #[test]
    fn compute_rectangle_area_06() {
        assert_eq!(compute_rectangle_area((7, 3), (2, 3)), 6);
    }

    #[test]
    fn compute_rectangle_area_07() {
        assert_eq!(compute_rectangle_area((11, 1), (2, 5)), 50);
    }
}

#[allow(clippy::too_many_lines)]
fn fold(coord_list: &[Coords]) -> u64 {
    let mut tmp_area = 0;
    // let mut winners = ((0, 0), (0, 0));
    let mut winners_area = 0;
    for i in 0..coord_list.len() {
        for j in (i + 1)..coord_list.len() {
            let candidate_area = compute_rectangle_area(coord_list[i], coord_list[j]);
            // println!(
            // "i: {:?}, j: {:?}, tmp_area: {tmp_area}, winners_area: {winners_area}, candidate_area: {candidate_area}",
            // coord_list[i], coord_list[j]
            // );
            if candidate_area > tmp_area {
                // winners = (coord_list[i], coord_list[j]);
                tmp_area = candidate_area;
                winners_area = candidate_area;
            }
        }
    }
    // println!(
    // "winners({:?},{:?}), area:{winners_area}",
    // winners.0, winners.1
    // );

    winners_area
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_sample() {
        let coord_list = parse_content(
            "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(fold(&coord_list), 50);
    }

    #[test]
    fn fold_final() {
        let coord_list = parse_content(&get_file_content("assets/input"));
        assert_eq!(fold(&coord_list), 4_294_870_800);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let coord_list = parse_content(&file_content);
    println!("Result: {}", fold(&coord_list));
}
