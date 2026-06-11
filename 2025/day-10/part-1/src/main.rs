use std::{cmp::Ordering, collections::HashMap, fs, ops::Sub};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct LightArray(Vec<bool>);

impl std::fmt::Display for LightArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|b| match b {
                    true => '#',
                    false => '.',
                })
                .collect::<String>()
        )
    }
}

impl Sub for LightArray {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        LightArray(
            self.0
                .iter()
                .zip(rhs.0)
                .map(|(t, o)| match (t, o) {
                    (true, false) | (false, true) => true,
                    (true, true) | (false, false) => false,
                    // Decomment this if 0-1 should be 1 and not 0
                    // (true, false) => true,
                    // (true | false, true) | (false, false) => false,
                })
                .collect(),
        )
    }
}

impl PartialOrd for LightArray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..self.0.len() {
            match (self.0[i], other.0[i]) {
                (true, false) => return Some(Ordering::Less),
                (false, true) => return Some(Ordering::Greater),
                _ => {}
            }
        }
        Some(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq)]
struct Machine {
    indicator_light_diagram: LightArray,
    button_wiring_schematic_list: Vec<LightArray>,
    joltage_requirements: Vec<u32>,
}

impl TryFrom<&str> for Machine {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut split = line.split_whitespace();
        let indicator_light_diagram: LightArray = {
            let mut list = vec![];
            for c in split.next().expect("bouh ! 🥲").chars() {
                match c {
                    '.' => list.push(false),
                    '#' => list.push(true),
                    ']' | '[' => {}
                    _ => panic!("wat"),
                }
            }
            list.reverse();
            LightArray(list)
        };
        let zero: Vec<bool> = indicator_light_diagram
            .0
            .clone()
            .iter()
            .map(|_| false)
            .collect();
        Ok(Self {
            indicator_light_diagram: indicator_light_diagram.clone(),
            button_wiring_schematic_list: {
                let mut list = vec![];
                for block in split {
                    if !block.contains('{') {
                        list.push({
                            let mut tmp_indicator_light_diagram = zero.clone();
                            for c in block.chars() {
                                match c {
                                    '(' | ')' | ',' => {}
                                    integer => {
                                        tmp_indicator_light_diagram[integer
                                            .to_digit(10)
                                            .unwrap_or_else(|| panic!("Not an integer {integer}"))
                                            as usize] = true;
                                    }
                                }
                            }
                            tmp_indicator_light_diagram.reverse();
                            LightArray(tmp_indicator_light_diagram)
                        });
                    }
                }
                list.sort_by(|a, b| a.partial_cmp(b).expect("Should be ordered"));
                list
            },
            joltage_requirements: vec![],
        })
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} {{{}}}",
            self.indicator_light_diagram
                .0
                .iter()
                .rev()
                .map(|x| match x {
                    true => '#',
                    false => '.',
                })
                .collect::<String>(),
            self.button_wiring_schematic_list
                .iter()
                .map(|x| format!(
                    "({})",
                    x.0.iter()
                        .enumerate()
                        .filter_map(|(i, v)| match v {
                            true => Some((x.0.len() - i - 1).to_string()),
                            false => None,
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                ))
                .collect::<Vec<String>>()
                .join(" "),
            self.joltage_requirements
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_debug {
    use super::*;

    #[test]
    fn parse_machine_to_string_01() {
        let machine =
            Machine::try_from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(
            machine.to_string(),
            "[.##.] (3,2) (3,1) (3) (2,0) (2) (1,0) {}"
        );
    }
    #[test]
    fn parse_machine_to_string_02() {
        let machine =
            Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        assert_eq!(
            machine.to_string(),
            "[...#.] (4,3,2,1) (4,3,2,0) (4,0) (3,2) (2,1,0) {}"
        );
    }
}

fn parse_content(content: &str) -> Vec<Machine> {
    content
        .lines()
        .map(|line| Machine::try_from(line).unwrap_or_else(|_| panic!("Not a machine {line}")))
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_sample() {
        let machine_list = parse_content(
            "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
",
        );
        assert_eq!(
            machine_list,
            vec![
                Machine {
                    indicator_light_diagram: LightArray(vec![false, true, true, false]),
                    button_wiring_schematic_list: vec![
                        LightArray(vec![true, true, false, false]),
                        LightArray(vec![true, false, true, false]),
                        LightArray(vec![true, false, false, false]),
                        LightArray(vec![false, true, false, true]),
                        LightArray(vec![false, true, false, false]),
                        LightArray(vec![false, false, true, true])
                    ],
                    joltage_requirements: vec![]
                },
                Machine {
                    indicator_light_diagram: LightArray(vec![false, true, false, false, false]),
                    button_wiring_schematic_list: vec![
                        LightArray(vec![true, true, true, true, false]),
                        LightArray(vec![true, true, true, false, true]),
                        LightArray(vec![true, false, false, false, true]),
                        LightArray(vec![false, true, true, false, false]),
                        LightArray(vec![false, false, true, true, true])
                    ],
                    joltage_requirements: vec![]
                },
                Machine {
                    indicator_light_diagram: LightArray(vec![true, false, true, true, true, false]),
                    button_wiring_schematic_list: vec![
                        LightArray(vec![true, true, false, true, true, true]),
                        LightArray(vec![false, true, true, true, true, true]),
                        LightArray(vec![false, true, true, false, false, true]),
                        LightArray(vec![false, false, false, true, true, false])
                    ],
                    joltage_requirements: vec![]
                }
            ]
        );
    }
}

#[derive(Debug)]
struct Count(usize);

type Cache = HashMap<(LightArray, Vec<LightArray>), usize>;

impl Machine {
    fn decompose(&self) -> usize {
        let zero: Vec<bool> = self
            .indicator_light_diagram
            .0
            .clone()
            .iter()
            .map(|_| false)
            .collect();

        let mut global_combination_min_count = Count(usize::MAX);

        let mut cache: Cache = Cache::new();

        recurse_decompose(
            &mut cache,
            &self.indicator_light_diagram,
            &self.button_wiring_schematic_list,
            &zero,
            0,
            &mut global_combination_min_count,
        )
    }
}

fn recurse_decompose(
    cache: &mut Cache,
    indicator_light_diagram: &LightArray,
    button_wiring_schematic_list: &[LightArray],
    zero: &[bool],
    current_combination_count: usize,
    global_combination_min_count: &mut Count,
) -> usize {
    if let Some(v) = cache.get(&(
        indicator_light_diagram.clone(),
        button_wiring_schematic_list.to_vec(),
    )) {
        return *v;
    }

    if current_combination_count >= global_combination_min_count.0 {
        return usize::MAX;
    }
    if indicator_light_diagram.0 == zero {
        global_combination_min_count.0 = current_combination_count;
        return current_combination_count;
    }
    if button_wiring_schematic_list.is_empty() {
        return usize::MAX;
    }

    let mut tmp_min_count = usize::MAX;

    for (index, button) in button_wiring_schematic_list.iter().enumerate() {
        let result = indicator_light_diagram.clone() - button.clone();
        let mut rest = button_wiring_schematic_list.to_vec();
        rest.remove(index);
        let returned_value = recurse_decompose(
            cache,
            &result,
            &rest,
            zero,
            current_combination_count + 1,
            global_combination_min_count,
        );
        if returned_value < tmp_min_count {
            tmp_min_count = returned_value;
            if tmp_min_count < current_combination_count {
                break;
            }
        }
    }
    cache.insert(
        (
            indicator_light_diagram.clone(),
            button_wiring_schematic_list.to_vec().clone(),
        ),
        tmp_min_count,
    );
    tmp_min_count
}

#[cfg(test)]
mod tests_decompose {
    use super::*;

    #[test]
    fn decompose_machine_01() {
        let machine =
            Machine::try_from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(machine.decompose(), 2);
    }

    #[test]
    fn decompose_machine_02() {
        let machine =
            Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        assert_eq!(machine.decompose(), 3);
    }

    #[test]
    fn decompose_machine_03() {
        let machine =
            Machine::try_from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
                .unwrap();
        assert_eq!(machine.decompose(), 2);
    }

    #[test]
    fn decompose_machine_04() {
        let machine =
    Machine::try_from("[.#.##.##.#] (9,8,6,4,3,2) (9,7,6,5,4,3,1) (9,7,5,3,1,0) (9,4,3,2) (9,3) (8,7,6,4,2,0) (8,5,3,2,1,0) (8,5,3,2,1) (7,6,5,2,1,0) (7,6,4,1) (7,4,2,1) (7,1) (0) {}")
    .unwrap();
        assert_eq!(machine.decompose(), 2);
    }
}

fn fold(machine_list: &[Machine]) -> usize {
    machine_list.iter().map(Machine::decompose).sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_sample() {
        let machine_list = parse_content(
            "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
",
        );
        assert_eq!(fold(&machine_list), 7);
    }

    #[test]
    fn fold_final() {
        let machine_list = parse_content(&get_file_content("assets/input"));
        assert_eq!(fold(&machine_list), 438);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let machine_list = parse_content(&file_content);
    println!("Result: {}", fold(&machine_list));
}
