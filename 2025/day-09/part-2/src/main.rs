use std::{collections::HashMap, fs};

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
        let coords_list = parse_content(
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
            coords_list,
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

type Segment = (Coords, Coords);

fn get_polygon(coords_list: &[Coords]) -> Vec<Segment> {
    let mut polygon: Vec<Segment> = vec![];

    for i in 0..(coords_list.len() - 1) {
        let a = coords_list[i];
        let b = coords_list[i + 1];
        polygon.push((a, b));
    }
    let a = coords_list[coords_list.len() - 1];
    let b = coords_list[0];
    polygon.push((a, b));

    polygon
}

#[cfg(test)]
mod tests_get_segments {
    use super::*;

    #[test]
    fn get_segments_sample() {
        let coords_list = parse_content(
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
            get_polygon(&coords_list),
            vec![
                ((7, 1), (11, 1)),
                ((11, 1), (11, 7)),
                ((11, 7), (9, 7)),
                ((9, 7), (9, 5)),
                ((9, 5), (2, 5)),
                ((2, 5), (2, 3)),
                ((2, 3), (7, 3)),
                ((7, 3), (7, 1))
            ],
        );
    }
}

fn is_point_on_any_segment(segments: &[(Coords, Coords)], point: &Coords) -> bool {
    // Points that are located ON segments
    for ((xa, ya), (xb, yb)) in segments {
        if point.0 == *xa && xa == xb {
            if ya < yb {
                if point.1 >= *ya && point.1 <= *yb {
                    return true;
                }
            } else if point.1 >= *yb && point.1 <= *ya {
                return true;
            }
        }
        if point.1 == *ya && ya == yb {
            if xa < xb {
                if point.0 >= *xa && point.0 <= *xb {
                    return true;
                }
            } else if point.0 >= *xb && point.0 <= *xa {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests_is_point_on_any_segment {
    use super::*;

    #[test]
    fn is_point_on_any_segment_on_polygon() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(is_point_on_any_segment(&polygon, &(9, 1)));
    }

    #[test]
    fn is_point_on_any_segment_on_vertical_segment() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(is_point_on_any_segment(&polygon, &(7, 2)));
    }
}

fn is_point_inside_polygon(
    raycast_count: &mut RaycastCount,
    cache: &mut Cache,
    polygon: &[Segment],
    point: &Coords,
    (max_x, max_y): &(u64, u64),
) -> bool {
    if let Some(&b) = cache.get(point) {
        // println!("CACHED");
        return b;
    }
    let mut result: bool = false;

    // if polygon.iter().any(|(a, b)| a == point || b == point) {
    // result = true;
    // }
    if !result && is_point_on_any_segment(polygon, point) {
        result = true;
    }

    if !result {
        raycast_count.0 += 1;
        let mut intersection = 0;

        if point.1 > point.0 {
            if point.1 > (max_y / 2) {
                for y in (point.1..*max_y).rev() {
                    if is_point_on_any_segment(polygon, &(point.0, y)) {
                        intersection += 1;
                    }
                }
            } else {
                for y in 0..point.1 {
                    if is_point_on_any_segment(polygon, &(point.0, y)) {
                        intersection += 1;
                    }
                }
            }
        } else if point.0 > (max_x / 2) {
            for x in (point.0..*max_x).rev() {
                if is_point_on_any_segment(polygon, &(x, point.1)) {
                    intersection += 1;
                }
            }
        } else {
            for x in 0..point.0 {
                if is_point_on_any_segment(polygon, &(x, point.1)) {
                    intersection += 1;
                }
            }
        }
        result = intersection % 2 == 1;
    }

    cache.insert(*point, result);

    result
}

#[cfg(test)]
mod tests_is_point_inside_polygon {
    use super::*;

    #[test]
    fn is_point_inside_polygon_when_on_a_segment() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(is_point_inside_polygon(
            &mut RaycastCount(0),
            &mut HashMap::new(),
            &polygon,
            &(9, 1),
            &(12, 12)
        ));
    }

    #[test]
    fn is_point_inside_polygon_when_inside() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(is_point_inside_polygon(
            &mut RaycastCount(0),
            &mut HashMap::new(),
            &polygon,
            &(9, 2),
            &(12, 12)
        ));
    }

    #[test]
    fn is_point_inside_polygon_when_outside() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(!is_point_inside_polygon(
            &mut RaycastCount(0),
            &mut HashMap::new(),
            &polygon,
            &(0, 0),
            &(12, 12)
        ));
    }

    #[test]
    fn is_point_inside_polygon_when_borderline_between_horizontal_and_vertical() {
        let coords_list = parse_content(
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
        let polygon = get_polygon(&coords_list);
        assert!(is_point_inside_polygon(
            &mut RaycastCount(0),
            &mut HashMap::new(),
            &polygon,
            &(3, 5),
            &(12, 12)
        ));
    }
}

fn is_point_inside_rectangle(strict: bool, rectangle: &[Coords; 4], (x, y): &Coords) -> bool {
    if strict {
        *x > rectangle[0].0 && *x < rectangle[1].0 && *y > rectangle[0].1 && *y < rectangle[2].1
    } else {
        *x >= rectangle[0].0 && *x <= rectangle[1].0 && *y >= rectangle[0].1 && *y <= rectangle[2].1
    }
}

#[cfg(test)]
mod tests_is_point_inside_rectangle {
    use super::*;

    #[test]
    fn is_point_inside_rectangle_01() {
        assert!(is_point_inside_rectangle(
            false,
            &[(2, 3), (9, 3), (9, 7), (2, 7)],
            &(9, 4)
        ));
    }

    #[test]
    fn is_point_inside_rectangle_02() {
        assert!(!is_point_inside_rectangle(
            true,
            &[(2, 3), (9, 3), (9, 7), (2, 7)],
            &(9, 4)
        ));
    }
}

/// Creates an oriented rectangle, given three points of said rectangle.
///
/// The fourth point is computed, and the four points are given with a predictable
/// orientation A, B, C, D, as follows:
///          A --->--- B
///          |         |
///          ∧         v
///          |         |
///          D ---<--- C
fn create_oriented_rectangle(rectangle: &[Coords; 2]) -> [Coords; 4] {
    let mut x = [rectangle[0].0, rectangle[1].0];
    x.sort_unstable();
    let min_x = x[0];
    let max_x = x[1];
    let mut y = [rectangle[0].1, rectangle[1].1];
    y.sort_unstable();
    let min_y = y[0];
    let max_y = y[1];

    [
        (min_x, min_y),
        (max_x, min_y),
        (max_x, max_y),
        (min_x, max_y),
    ]
}

type Cache = HashMap<Coords, bool>;

/// Determine whether a rectangle is a "valid" one.
fn is_valid_rectangle(
    cache: &mut Cache,
    raycast_count: &mut RaycastCount,
    rectangle: &[Coords; 2],
    red_tiles_list: &[Coords],
    polygon: &[Segment],
    bounding_rect: &(u64, u64),
) -> bool {
    let candidate_rectangle = create_oriented_rectangle(rectangle);

    // println!("\nInput rectangle {rectangle:?}");
    // println!(
    // "Checking rectangle {candidate_rectangle:?} which has area {}",
    // compute_rectangle_area(rectangle)
    // );
    // println!("red_tiles_list {red_tiles_list:?}");

    for red_tile in red_tiles_list {
        let mut temp_tile: Option<Coords> = None;
        if is_point_inside_rectangle(true, &candidate_rectangle, red_tile) {
            temp_tile = Some(*red_tile);
        }
        if temp_tile.is_none()
            && red_tile.0 > candidate_rectangle[0].0
            && red_tile.0 < candidate_rectangle[2].0
        {
            temp_tile = if red_tile.1 < candidate_rectangle[0].1 {
                Some((red_tile.0, candidate_rectangle[0].1))
            } else {
                Some((red_tile.0, candidate_rectangle[2].1))
            };
        }

        if temp_tile.is_none()
            && red_tile.1 > candidate_rectangle[0].1
            && red_tile.1 < candidate_rectangle[2].1
        {
            temp_tile = if red_tile.0 < candidate_rectangle[0].0 {
                Some((candidate_rectangle[0].0, red_tile.1))
            } else {
                Some((candidate_rectangle[2].0, red_tile.1))
            };
        }

        if let Some(tile) = temp_tile {
            // println!("tile: {:?}", &tile);
            let cross_points = get_cross_points(&tile);
            // println!("cross_points: {:?}", &cross_points);
            for cross_point in cross_points {
                if is_point_inside_rectangle(false, &candidate_rectangle, &cross_point)
                    && !is_point_inside_polygon(
                        raycast_count,
                        cache,
                        polygon,
                        &cross_point,
                        bounding_rect,
                    )
                {
                    // println!("CROSS POINT IS NOT INSIDE POLYGON {cross_point:?}");
                    return false;
                }
            }
        }
    }

    true
}

/// For a given X, return a,b,c,d such as :
///     a b
///      X
///     c d
fn get_cross_points(point: &(u64, u64)) -> [(u64, u64); 4] {
    [
        (point.0 - 1, point.1 - 1),
        (point.0 - 1, point.1 + 1),
        (point.0 + 1, point.1 - 1),
        (point.0 + 1, point.1 + 1),
    ]
}

fn compute_rectangle_area(coords_list: &[Coords; 2]) -> u64 {
    let (xa, ya) = coords_list[0];
    let (xb, yb) = coords_list[1];

    (xb.abs_diff(xa) + 1) * (yb.abs_diff(ya) + 1)
}

#[cfg(test)]
mod tests_compute_rectangle_area {
    use super::*;

    #[test]
    fn compute_rectangle_area_01() {
        assert_eq!(compute_rectangle_area(&[(1, 1), (2, 2)]), 4);
    }

    #[test]
    fn compute_rectangle_area_02() {
        assert_eq!(compute_rectangle_area(&[(0, 0), (1, 1)]), 4);
    }

    #[test]
    fn compute_rectangle_area_03() {
        assert_eq!(compute_rectangle_area(&[(1, 1), (0, 0)]), 4);
    }

    #[test]
    fn compute_rectangle_area_04() {
        assert_eq!(compute_rectangle_area(&[(7, 3), (11, 1)]), 15);
    }

    #[test]
    fn compute_rectangle_area_05() {
        assert_eq!(compute_rectangle_area(&[(9, 5), (9, 7)]), 3);
    }

    #[test]
    fn compute_rectangle_area_06() {
        assert_eq!(compute_rectangle_area(&[(2, 3), (9, 5)]), 24);
    }
}

struct RaycastCount(u64);

#[allow(clippy::too_many_lines)]
fn fold(red_tiles_list: &[Coords]) -> u64 {
    let mut cache: Cache = HashMap::new();
    let polygon = get_polygon(red_tiles_list);
    // let mut winners = ((0, 0), (0, 0));
    let mut winners_area = 0;
    let mut count = 0;
    let mut candidate_rectangle_list: Vec<((Coords, Coords), u64)> = vec![];

    let mut max_x = 0;
    let mut max_y = 0;

    for i in 0..red_tiles_list.len() {
        for j in (i + 1)..red_tiles_list.len() {
            if red_tiles_list[i].0 > max_x {
                max_x = red_tiles_list[i].0;
            }
            if red_tiles_list[i].1 > max_y {
                max_y = red_tiles_list[i].1;
            }
            let candidate_area = compute_rectangle_area(&[red_tiles_list[i], red_tiles_list[j]]);
            candidate_rectangle_list.push(((red_tiles_list[i], red_tiles_list[j]), candidate_area));
        }
    }

    max_x += 1;
    max_y += 1;

    println!(
        "\nCandidate rectangles: {}\n",
        candidate_rectangle_list.len()
    );
    candidate_rectangle_list.sort_unstable_by_key(|c| c.1);

    let mut raycast_count: RaycastCount = RaycastCount(0);

    for (candidate_rectangle, area) in candidate_rectangle_list.iter().rev() {
        count += 1;
        if count % 100 == 0 {
            println!("{count}");
        }
        if is_valid_rectangle(
            &mut cache,
            &mut raycast_count,
            &[candidate_rectangle.0, candidate_rectangle.1],
            red_tiles_list,
            &polygon,
            &(max_x, max_y),
        ) {
            winners_area = *area;
            break;
        }
    }

    println!("RaycastCount: {}", raycast_count.0);
    println!("Count: {count}");

    winners_area
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_sample() {
        let coords_list = parse_content(
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
        assert_eq!(fold(&coords_list), 24);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let coords_list = parse_content(&file_content);
    println!("Result: {}", fold(&coords_list));
}
