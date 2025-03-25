use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

pub fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coords = (isize, isize);

#[derive(Debug, PartialEq, Clone)]
pub struct Map(HashMap<Coords, char>);

impl Map {
    pub fn new() -> Self {
        Map(HashMap::new())
    }

    pub fn get_bounding_rect(&self) -> (isize, isize) {
        let mut max_x = 0;
        let mut max_y = 0;

        for ((x, y), _) in self.0.clone() {
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        (max_x, max_y)
    }

    pub fn get(&self, coords: &Coords) -> Option<&char> {
        self.0.get(coords)
    }

    fn insert(&mut self, key: (isize, isize), value: char) -> Option<char> {
        self.0.insert(key, value)
    }
}

impl From<Vec<Vec<char>>> for Map {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self(HashMap::from_iter(
            value
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(x, c)| ((x as isize, y as isize), *c))
                        .collect::<Vec<(Coords, char)>>()
                })
                .collect::<Vec<(Coords, char)>>(),
        ))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| ((x as isize, y as isize), c))
                        .collect::<Vec<(Coords, char)>>()
                })
                .collect(),
        )
    }
}

/// Display given map, and will highlight given region,
/// and will show given fences.
#[allow(dead_code)]
fn display_map(exploded_map: &Map, fences: &HashSet<Coords>) {
    let (max_x, max_y) = exploded_map.get_bounding_rect();

    for y in 0..=max_y + 1 {
        for x in 0..=max_x + 1 {
            if fences.contains(&(x, y)) {
                print!("*");
            } else if let Some(plant) = exploded_map.get(&(x, y)) {
                print!("{}", plant);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests_map {
    use super::*;

    #[test]
    fn map_from_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        assert_eq!(
            map,
            Map::from(vec![
                vec!['A', 'A', 'A', 'A',],
                vec!['B', 'B', 'C', 'D',],
                vec!['B', 'B', 'C', 'C',],
                vec!['E', 'E', 'E', 'C',],
            ])
        );
    }
}

/// explode is meant to convert a map to its "exploded" form.
///
/// For instance:
/// AAAA
/// BBCD
/// BBCC
/// EEEC
///
/// becomes:
/// .........
/// .A.A.A.A.
/// .........
/// .B.B.C.D.
/// .........
/// .B.B.C.C.
/// .........
/// .E.E.E.C.
/// .........
pub fn get_exploded_map(map: &Map) -> Map {
    let (max_x, max_y) = map.get_bounding_rect();
    let mut exploded_map = Map::new();

    for y in (0..=max_y).rev() {
        for x in (0..=max_x).rev() {
            let new_x = x * 2 + 1;
            exploded_map.insert(
                (new_x, y * 2 + 1),
                map.get(&(x, y)).expect("Should not happen !").to_owned(),
            );
        }
    }
    exploded_map
}

/// Same as get_exploded_map, but for a region
pub fn get_exploded_region(region: &Region) -> Region {
    let mut exploded_region = Region::new();

    for (x, y) in region.0.clone() {
        let new_x = x * 2 + 1;
        exploded_region.insert((new_x, y * 2 + 1));
    }
    exploded_region
}

#[cfg(test)]
mod tests_map_explode {
    use super::*;

    #[test]
    fn explode_simple() {
        let map = Map::from(
            "\
AAAA
",
        );
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            exploded_map,
            Map(HashMap::from_iter(vec![
                ((1, 1), 'A'),
                ((3, 1), 'A'),
                ((5, 1), 'A'),
                ((7, 1), 'A'),
            ]))
        );
    }

    #[test]
    fn explode_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            exploded_map,
            Map(HashMap::from_iter(vec![
                ((1, 1), 'A'),
                ((3, 1), 'A'),
                ((5, 1), 'A'),
                ((7, 1), 'A'),
                ((1, 3), 'B'),
                ((3, 3), 'B'),
                ((1, 5), 'B'),
                ((3, 5), 'B'),
                ((5, 3), 'C'),
                ((5, 5), 'C'),
                ((7, 5), 'C'),
                ((7, 7), 'C'),
                ((7, 3), 'D'),
                ((1, 7), 'E'),
                ((3, 7), 'E'),
                ((5, 7), 'E')
            ]))
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Region(pub HashSet<Coords>);

impl Region {
    fn new() -> Region {
        Region(HashSet::new())
    }

    #[allow(dead_code)]
    fn from_iter(v: Vec<(isize, isize)>) -> Region {
        Region(HashSet::from_iter(v))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, coords: &(isize, isize)) -> bool {
        self.0.contains(coords)
    }

    fn insert(&mut self, coords: (isize, isize)) -> bool {
        self.0.insert(coords)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_bounding_rect(&self) -> ((isize, isize), (isize, isize)) {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for (x, y) in self.0.clone() {
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
        }

        ((min_x, min_y), (max_x, max_y))
    }
}

pub fn get_regions(map: &Map) -> Vec<Region> {
    let (max_x, max_y) = map.get_bounding_rect();
    let mut regions: Vec<Region> = vec![];

    let mut current_region = Region::new();
    let mut explored_plots: HashSet<Coords> = HashSet::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(plant) = map.get(&(x, y)) {
                if !explored_plots.contains(&(x, y)) {
                    if !current_region.is_empty() {
                        regions.push(current_region.to_owned());
                    }
                    let current_plant = *plant;
                    current_region = Region::new();

                    let mut queue: VecDeque<Coords> = VecDeque::from(vec![(x, y)]);

                    while let Some(coords) = queue.pop_front() {
                        // Keep an explored plot list to avoid infinite recursion
                        if let Some(temp_plant) = map.get(&coords) {
                            if *temp_plant == current_plant && !explored_plots.contains(&coords) {
                                explored_plots.insert(coords);
                                current_region.insert(coords);
                                queue.push_back((coords.0 - 1, coords.1));
                                queue.push_back((coords.0 + 1, coords.1));
                                queue.push_back((coords.0, coords.1 - 1));
                                queue.push_back((coords.0, coords.1 + 1));
                            }
                        }
                    }
                }
            }
        }
    }
    if !current_region.is_empty() {
        regions.push(current_region.to_owned());
    }

    regions
}

#[cfg(test)]
mod tests_get_regions {
    use super::*;

    #[test]
    fn get_regions_trivial() {
        let map = Map::from(
            "\
AA
BC
",
        );
        assert_eq!(
            get_regions(&map),
            vec![
                Region::from_iter(vec![(0, 0), (1, 0)]),
                Region::from_iter(vec![(0, 1)]),
                Region::from_iter(vec![(1, 1)])
            ]
        );
    }

    #[test]
    fn get_regions_simple() {
        let map = Map::from(
            "\
AAA
BBC
BBC
",
        );
        assert_eq!(
            get_regions(&map),
            vec![
                Region::from_iter(vec![(0, 0), (1, 0), (2, 0)]),
                Region::from_iter(vec![(0, 1), (1, 1), (0, 2), (1, 2)]),
                Region::from_iter(vec![(2, 1), (2, 2)]),
            ]
        );
    }

    #[test]
    fn get_regions_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        assert_eq!(
            get_regions(&map),
            vec![
                Region::from_iter(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
                Region::from_iter(vec![(0, 1), (1, 1), (0, 2), (1, 2)]),
                Region::from_iter(vec![(2, 1), (2, 2), (3, 2), (3, 3)]),
                Region::from_iter(vec![(3, 1)]),
                Region::from_iter(vec![(0, 3), (1, 3), (2, 3)]),
            ]
        );
    }

    #[test]
    fn get_regions_intertwined() {
        let map = Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let regions = get_regions(&map);
        assert_eq!(regions.len(), 5);
    }

    #[test]
    fn get_regions_final() {
        let map = Map::from(
            "\
IIIIIICCFF
RRRRIICCCC
VVRRRCCCCC
VVRCCCJCCC
VVVVCJJCRE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        let regions = get_regions(&map);
        assert_eq!(regions.len(), 11);
    }
}

pub fn get_area(region: &Region) -> usize {
    region.len()
}

#[cfg(test)]
mod tests_get_area {
    use super::*;

    #[test]
    fn get_area_not_so_simple() {
        let regions = get_regions(&Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        ));

        assert_eq!(get_area(&regions[0]), 4);
        assert_eq!(get_area(&regions[1]), 4);
        assert_eq!(get_area(&regions[2]), 4);
        assert_eq!(get_area(&regions[3]), 1);
        assert_eq!(get_area(&regions[4]), 3);
    }

    #[test]
    fn get_area_intertwined() {
        let regions = get_regions(&Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        ));

        assert_eq!(get_area(&regions[0]), 21);
        assert_eq!(get_area(&regions[1]), 1);
        assert_eq!(get_area(&regions[2]), 1);
        assert_eq!(get_area(&regions[3]), 1);
        assert_eq!(get_area(&regions[4]), 1);
    }
}

pub fn get_perimeter(map: &Map, region: &Region) -> usize {
    let mut perimeter = 0;
    let first_element = region.0.clone().into_iter().next().expect("Empty Set");
    let region_plant = map.get(&first_element).expect("Region should not be empty");
    for (x, y) in region.0.clone() {
        let neighbours = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
        let mut current_perimeter = 4;

        neighbours
            .iter()
            .filter_map(|neighbour| map.get(neighbour))
            .for_each(|plant| {
                if plant == region_plant {
                    current_perimeter -= 1;
                }
            });
        perimeter += current_perimeter;
    }

    perimeter
}

#[cfg(test)]
mod tests_get_perimeter {
    use super::*;

    #[test]
    fn get_perimeter_solo() {
        let map = Map::from(
            "\
D
",
        );
        let regions = get_regions(&map);

        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[0]), 4);
    }

    #[test]
    fn get_perimeter_duo() {
        let map = Map::from(
            "\
DD
",
        );
        let regions = get_regions(&map);

        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[0]), 6);
    }

    #[test]
    fn get_perimeter_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        let regions = get_regions(&map);

        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[0]), 10);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[1]), 8);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[2]), 10);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[3]), 4);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[4]), 8);
    }

    #[test]
    fn get_perimeter_intertwined() {
        let map = Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let regions = get_regions(&map);

        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[0]), 36);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[1]), 4);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[2]), 4);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[3]), 4);
        pretty_assertions::assert_eq!(get_perimeter(&map, &regions[4]), 4);
    }
}

/// get_fences retrieves fences bits for an exploded region.
///
/// For instance:
/// .........
/// .A.A.A.A.
/// .........
/// .B.B.C.D.
/// .........
/// .B.B.C.C.
/// .........
/// .E.E.E.C.
/// .........
///
/// will yield:
///  - - - -
/// |A A A A|
///  - - - -
/// |B B|C|D|
///        -
/// |B B|C C|
///  - - -
/// |E E E|C|
///  - - - -
fn get_fences(exploded_map: &Map, exploded_region: &Region) -> HashSet<Coords> {
    let mut fences: HashSet<Coords> = HashSet::new();
    let region_plant = exploded_map
        .get(exploded_region.0.iter().next().expect("Empty region"))
        .expect("Region should not be empty");
    for (x, y) in exploded_region.0.clone() {
        let neighbours = [(x, y - 2), (x - 2, y), (x + 2, y), (x, y + 2)];

        neighbours
            .iter()
            .filter(|neighbour| match exploded_map.get(neighbour) {
                Some(neighbour_plant) => neighbour_plant != region_plant,
                None => true,
            })
            .for_each(|&(neighbour_x, neighbour_y)| {
                let new_fence = {
                    match (neighbour_x.cmp(&x), neighbour_y.cmp(&y)) {
                        (Ordering::Equal, Ordering::Less) => (x, neighbour_y + 1),
                        (Ordering::Equal, Ordering::Greater) => (x, neighbour_y - 1),
                        (Ordering::Less, Ordering::Equal) => (neighbour_x + 1, y),
                        (Ordering::Greater, Ordering::Equal) => (neighbour_x - 1, y),
                        _ => panic!("Should not happen"),
                    }
                };
                fences.insert(new_fence);
            });
    }

    fences
}

#[cfg(test)]
mod tests_get_fences {
    use super::*;

    #[test]
    fn get_fences_trivial() {
        let map = Map::from(
            "\
D
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        let fences = get_fences(&exploded_map, &get_exploded_region(&regions[0]));

        assert_eq!(
            fences,
            HashSet::from_iter(vec![(1, 0), (1, 2), (0, 1), (2, 1)])
        );
    }

    #[test]
    fn get_fences_simple() {
        let map = Map::from(
            "\
AAAA
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        let fences = get_fences(&exploded_map, &get_exploded_region(&regions[0]));

        assert_eq!(
            fences,
            HashSet::from_iter(vec![
                (1, 0),
                (3, 0),
                (5, 0),
                (7, 0),
                (1, 2),
                (3, 2),
                (5, 2),
                (7, 2),
                (0, 1),
                (8, 1)
            ])
        );
    }

    #[test]
    fn get_fences_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        let fences = get_fences(&exploded_map, &get_exploded_region(&regions[0]));

        assert_eq!(
            fences,
            HashSet::from_iter(vec![
                (1, 0),
                (3, 0),
                (5, 0),
                (7, 0),
                (1, 2),
                (3, 2),
                (5, 2),
                (7, 2),
                (0, 1),
                (8, 1)
            ])
        );
    }
}

#[derive(PartialEq)]
enum InOut {
    UnSet,
    Inside,
    Outside,
}

/// get_sides is meant to count the plot "sides".
///
/// For instance:
///  - - - -
/// |A A A A|
///  - - - -
///  will return four sides (top, left, right, and bottom).
///
///  A "side" can have any number of fences, but must be contiguous, and straight
pub fn get_sides(exploded_map: &Map, exploded_region: &Region) -> usize {
    let fences = get_fences(exploded_map, exploded_region);
    let region_plant = exploded_map
        .get(exploded_region.0.iter().next().expect("Empty region"))
        .expect("Region should not be empty");

    let mut sides = 0;

    let ((region_min_x, region_min_y), (region_max_x, region_max_y)) =
        exploded_region.get_bounding_rect();

    // Get horizontal sides
    let mut contiguous;
    let mut y = region_min_y - 1;
    let mut x;
    while y < region_max_y + 2 {
        x = region_min_x;
        contiguous = false;
        let mut in_out: InOut = InOut::UnSet;
        while x < region_max_x + 2 {
            if fences.contains(&(x, y)) {
                let bottom = exploded_map.get(&(x, y + 1)).unwrap_or(&'.');
                let top = exploded_map.get(&(x, y - 1)).unwrap_or(&'.');
                if !contiguous {
                    sides += 1;
                    contiguous = true;
                    if bottom == region_plant {
                        in_out = InOut::Outside;
                    }
                    if top == region_plant {
                        in_out = InOut::Inside;
                    }
                } else if (bottom == region_plant && in_out == InOut::Inside)
                    || (top == region_plant && in_out == InOut::Outside)
                {
                    sides += 1;
                    if bottom == region_plant {
                        in_out = InOut::Outside;
                    }
                    if top == region_plant {
                        in_out = InOut::Inside;
                    }
                }
            } else {
                contiguous = false;
                in_out = InOut::UnSet;
            }
            x += 2;
        }
        y += 2;
    }

    // Get vertical sides
    let mut contiguous;
    let mut x = region_min_x - 1;
    let mut y;
    while x < region_max_x + 2 {
        y = region_min_y;
        contiguous = false;
        let mut in_out: InOut = InOut::UnSet;
        while y < region_max_y + 2 {
            if fences.contains(&(x, y)) {
                let bottom = exploded_map.get(&(x + 1, y)).unwrap_or(&'.');
                let top = exploded_map.get(&(x - 1, y)).unwrap_or(&'.');
                if !contiguous {
                    sides += 1;
                    contiguous = true;
                    if bottom == region_plant {
                        in_out = InOut::Outside;
                    }
                    if top == region_plant {
                        in_out = InOut::Inside;
                    }
                } else if (bottom == region_plant && in_out == InOut::Inside)
                    || (top == region_plant && in_out == InOut::Outside)
                {
                    sides += 1;
                    if bottom == region_plant {
                        in_out = InOut::Outside;
                    }
                    if top == region_plant {
                        in_out = InOut::Inside;
                    }
                }
            } else {
                contiguous = false;
                in_out = InOut::UnSet;
            }
            y += 2;
        }
        x += 2;
    }

    sides
}

#[cfg(test)]
mod tests_get_sides {
    use super::*;

    #[test]
    fn get_sides_solo() {
        let map = Map::from(
            "\
D
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            4
        );
    }

    #[test]
    fn get_sides_duo() {
        let map = Map::from(
            "\
DD
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            4
        );
    }

    #[test]
    fn get_sides_quad() {
        let map = Map::from(
            "\
QQQQ
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            4
        );
    }

    #[test]
    fn get_sides_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[1])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[2])),
            8
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[3])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[4])),
            4
        );
    }

    #[test]
    fn get_sides_intertwined() {
        let map = Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            20
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[1])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[2])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[3])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[4])),
            4
        );
    }

    #[test]
    fn get_sides_donut() {
        let map = Map::from(
            "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);

        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[0])),
            12
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[1])),
            4
        );
        assert_eq!(
            get_sides(&exploded_map, &get_exploded_region(&regions[2])),
            4
        );
    }
}

pub fn calc_fence_price(exploded_map: &Map, exploded_region: &Region) -> usize {
    get_area(exploded_region) * get_sides(exploded_map, exploded_region)
}

#[cfg(test)]
mod tests_calc_fence_price {
    use super::*;

    #[test]
    fn calc_fence_price_solo() {
        let map = Map::from(
            "\
D
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[0])),
            4
        );
    }

    #[test]
    fn calc_fence_price_duo() {
        let map = Map::from(
            "\
DD
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[0])),
            8
        );
    }

    #[test]
    fn calc_fence_price_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[0])),
            16
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[1])),
            16
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[2])),
            32
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[3])),
            4
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[4])),
            12
        );
    }

    #[test]
    fn calc_fence_price_intertwined() {
        let map = Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[0])),
            420
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[1])),
            4
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[2])),
            4
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[3])),
            4
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[4])),
            4
        );
    }

    #[test]
    fn calc_fence_price_big() {
        let map = Map::from(
            "\
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    ",
        );
        let regions = get_regions(&map);
        let exploded_map = get_exploded_map(&map);
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[0])),
            120
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[1])),
            16
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[2])),
            308
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[3])),
            120
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[4])),
            130
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[5])),
            132
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[6])),
            4
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[7])),
            104
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[8])),
            224
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[9])),
            30
        );
        assert_eq!(
            calc_fence_price(&exploded_map, &get_exploded_region(&regions[10])),
            18
        );
    }
}

pub fn fold_part1(map: &Map) -> usize {
    get_regions(map)
        .iter()
        .map(|region| get_area(region) * get_perimeter(map, region))
        .sum()
}

#[cfg(test)]
mod tests_fold_part1 {
    use super::*;

    #[test]
    fn fold_part1_duo() {
        let map = Map::from(
            "\
    DD
    ",
        );
        pretty_assertions::assert_eq!(fold_part1(&map), 12);
    }

    #[test]
    fn fold_part1_not_so_simple() {
        let map = Map::from(
            "\
    AAAA
    BBCD
    BBCC
    EEEC
    ",
        );
        pretty_assertions::assert_eq!(fold_part1(&map), 140);
    }

    #[test]
    fn fold_part1_intertwined() {
        let map = Map::from(
            "\
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    ",
        );
        pretty_assertions::assert_eq!(fold_part1(&map), 772);
    }

    #[test]
    fn fold_part1_big() {
        let map = Map::from(
            "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        pretty_assertions::assert_eq!(fold_part1(&map), 1930);
    }
    #[test]

    fn fold_part1_final() {
        let map = Map::from(get_file_content("assets/input").as_str());
        assert_eq!(fold_part1(&map), 1396298);
    }
}

pub fn fold_part2(map: &Map) -> usize {
    let exploded_map = get_exploded_map(map);
    get_regions(map)
        .iter()
        .map(|region| {
            let exploded_region = get_exploded_region(region);
            calc_fence_price(&exploded_map, &exploded_region)
        })
        .sum()
}

#[cfg(test)]
mod tests_fold_part2 {
    use super::*;

    #[test]
    fn fold_duo() {
        let map = Map::from(
            "\
DD
",
        );
        assert_eq!(fold_part2(&map), 8);
    }

    #[test]
    fn fold_not_so_simple() {
        let map = Map::from(
            "\
AAAA
BBCD
BBCC
EEEC
",
        );
        assert_eq!(fold_part2(&map), 80);
    }

    #[test]
    fn fold_intertwined() {
        let map = Map::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        assert_eq!(fold_part2(&map), 436);
    }

    #[test]
    fn fold_big() {
        let map = Map::from(
            "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        assert_eq!(fold_part2(&map), 1206);
    }

    #[test]
    fn fold_e_x() {
        let map = Map::from(
            "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        );
        assert_eq!(fold_part2(&map), 236);
    }

    #[test]
    fn fold_donut() {
        let map = Map::from(
            "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );
        assert_eq!(fold_part2(&map), 368);
    }

    #[test]
    fn fold_final() {
        let map = Map::from(get_file_content("assets/input").as_str());
        assert_eq!(fold_part2(&map), 853588);
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
