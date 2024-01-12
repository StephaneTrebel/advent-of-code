use std::fs;

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Coords {
    x: f64,
    y: f64,
}

type List = Vec<Hail>;

#[derive(Debug, PartialEq, Clone)]
struct Content {
    list: List,
}

fn parse_content(lines: &str) -> Content {
    let regex = Regex::new(r"(?<px>[0-9]+), (?<py>[0-9]+), (?<pz>[0-9]+) @ (?<vx>[-0-9]+), (?<vy>[-0-9]+), (?<vz>[-0-9]+)").unwrap();

    Content {
        list: lines
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let captures = regex.captures(&line).unwrap();
                let x: f64 = captures["px"].parse().unwrap();
                let y: f64 = captures["py"].parse().unwrap();
                Hail {
                    px: x,
                    py: y,
                    pz: captures["pz"].parse().unwrap(),
                    dx: captures["vx"].parse().unwrap(),
                    dy: captures["vy"].parse().unwrap(),
                    dz: captures["vz"].parse().unwrap(),
                }
            })
            .collect::<List>(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3
",
        );
        assert_eq!(
            content.list.get(3),
            Some(&Hail {
                px: 12.,
                py: 31.,
                pz: 28.,
                dx: -1.,
                dy: -2.,
                dz: -1.
            })
        );
    }
}

fn count_colliding_hailstones(
    content: &Content,
    top_left_corner: f64,
    bottom_right_corner: f64,
) -> usize {
    content
        .list
        .iter()
        .tuple_combinations::<(_, _)>()
        .par_bridge()
        .filter(|(h1, h2)| {
            // println!("Hail_1: {h1:?}, Hail_2: {h2:?}");

            let t2 = (h1.dx * (h2.py - h1.py) + h1.dy * (h1.px - h2.px))
                / (h2.dx * h1.dy - h2.dy * h1.dx);
            let t1 = (h2.dx * (h1.py - h2.py) + h2.dy * (h2.px - h1.px))
                / (h1.dx * h2.dy - h1.dy * h2.dx);

            // println!("{t1} {t2}");

            if t1 < 0. || t2 < 0. {
                // println!("Hailstones have intesected in the past");
                return false;
            }

            // Need only one since we are computing the precise moment the are
            // in the same place
            let target_x = h1.px + t1 * h1.dx;
            let target_y = h1.py + t1 * h1.dy;
            // println!("{target_x} {target_y}");
            if target_x >= top_left_corner
                && target_x <= bottom_right_corner
                && target_y >= top_left_corner
                && target_y <= bottom_right_corner
            {
                // println!("## OK ##");
                return true;
            } else {
                return false;
            }
        })
        .count()
}

#[cfg(test)]
mod tests_count_colliding_hailstones {
    use super::*;

    #[test]
    fn count_colliding_hailstones_01() {
        let hailstones = parse_content(
            &"\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3
",
        );
        assert_eq!(count_colliding_hailstones(&hailstones, 7., 27.), 2);
    }
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!(
        "Part 1: {}",
        count_colliding_hailstones(&content, 200000000000000., 400000000000000.)
    );
}
