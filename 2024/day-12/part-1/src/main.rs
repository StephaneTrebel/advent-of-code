use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn get_bounding_rect(&self) -> (isize, isize) {
        (self.0[0].len() as isize, self.0.len() as isize)
    }

    fn get(&self, (x, y): &Coords) -> Option<&char> {
        self.0
            .get(*y as usize)
            .and_then(|line| line.get(*x as usize))
    }
}

impl From<Vec<Vec<char>>> for Map {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self(value)
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|line| line.chars().collect())
                .collect(),
        )
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
        pretty_assertions::assert_eq!(
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

type Coords = (isize, isize);
type Region = Vec<Coords>;

fn get_regions(map: &Map) -> Vec<Region> {
    let (max_x, max_y) = map.get_bounding_rect();
    let mut regions: Vec<Region> = vec![];

    let mut current_region = Region::new();
    let mut explored_plots: HashSet<Coords> = HashSet::new();

    for y in 0..max_y {
        for x in 0..max_x {
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
                                current_region.push(coords);
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
        pretty_assertions::assert_eq!(
            get_regions(&map),
            vec![vec![(0, 0), (1, 0)], vec![(0, 1)], vec![(1, 1)],]
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
        pretty_assertions::assert_eq!(
            get_regions(&map),
            vec![
                vec![(0, 0), (1, 0), (2, 0)],
                vec![(0, 1), (1, 1), (0, 2), (1, 2)],
                vec![(2, 1), (2, 2)],
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
        pretty_assertions::assert_eq!(
            get_regions(&map),
            vec![
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 1), (1, 1), (0, 2), (1, 2)],
                vec![(2, 1), (2, 2), (3, 2), (3, 3)],
                vec![(3, 1)],
                vec![(0, 3), (1, 3), (2, 3)],
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

fn get_area(region: &Region) -> usize {
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

        pretty_assertions::assert_eq!(get_area(&regions[0]), 4);
        pretty_assertions::assert_eq!(get_area(&regions[1]), 4);
        pretty_assertions::assert_eq!(get_area(&regions[2]), 4);
        pretty_assertions::assert_eq!(get_area(&regions[3]), 1);
        pretty_assertions::assert_eq!(get_area(&regions[4]), 3);
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

        pretty_assertions::assert_eq!(get_area(&regions[0]), 21);
        pretty_assertions::assert_eq!(get_area(&regions[1]), 1);
        pretty_assertions::assert_eq!(get_area(&regions[2]), 1);
        pretty_assertions::assert_eq!(get_area(&regions[3]), 1);
        pretty_assertions::assert_eq!(get_area(&regions[4]), 1);
    }
}

fn get_perimeter(map: &Map, region: &Region) -> usize {
    let mut perimeter = 0;
    let region_plant = map.get(&region[0]).expect("Region should not be empty");
    for (x, y) in region {
        let neighbours = [(*x, y - 1), (x - 1, *y), (x + 1, *y), (*x, y + 1)];
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

fn calc_fence_price(map: &Map, region: &Region) -> usize {
    let area = get_area(region);
    let perimeter = get_perimeter(map, region);
    area * perimeter
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

        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[0]), 4);
    }

    #[test]
    fn calc_fence_price_duo() {
        let map = Map::from(
            "\
    DD
    ",
        );
        let regions = get_regions(&map);

        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[0]), 12);
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

        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[0]), 40);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[1]), 32);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[2]), 40);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[3]), 4);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[4]), 24);
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

        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[0]), 756);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[1]), 4);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[2]), 4);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[3]), 4);
        pretty_assertions::assert_eq!(calc_fence_price(&map, &regions[4]), 4);
    }
}

fn fold(map: &Map) -> usize {
    get_regions(map)
        .iter()
        .map(|region| calc_fence_price(map, region))
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_duo() {
        let map = Map::from(
            "\
    DD
    ",
        );
        pretty_assertions::assert_eq!(fold(&map), 12);
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
        pretty_assertions::assert_eq!(fold(&map), 140);
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
        pretty_assertions::assert_eq!(fold(&map), 772);
    }

    #[test]
    fn fold_final() {
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
        pretty_assertions::assert_eq!(fold(&map), 1930);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let map = Map::from(get_file_content("assets/input").as_str());

    println!("Result: {}", fold(&map));
}
