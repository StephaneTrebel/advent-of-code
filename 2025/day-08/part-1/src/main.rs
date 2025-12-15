use std::{collections::BTreeSet, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coords = (u32, u32, u32);

fn parse_content(content: &str) -> Vec<Coords> {
    content
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_final() {
        let coord_list = parse_content(
            "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );

        assert_eq!(
            coord_list,
            vec![
                (162, 817, 812),
                (57, 618, 57),
                (906, 360, 560),
                (592, 479, 940),
                (352, 342, 300),
                (466, 668, 158),
                (542, 29, 236),
                (431, 825, 988),
                (739, 650, 466),
                (52, 470, 668),
                (216, 146, 977),
                (819, 987, 18),
                (117, 168, 530),
                (805, 96, 715),
                (346, 949, 466),
                (970, 615, 88),
                (941, 993, 340),
                (862, 61, 35),
                (984, 92, 344),
                (425, 690, 689)
            ]
        );
    }
}

fn compute_euclidian_distance(first_box: &Coords, second_box: &Coords) -> i64 {
    (i64::from(first_box.0) - i64::from(second_box.0)).pow(2)
        + (i64::from(first_box.1) - i64::from(second_box.1)).pow(2)
        + (i64::from(first_box.2) - i64::from(second_box.2)).pow(2)
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests_compute_euclidian_distance {
    use super::*;

    #[test]
    fn compute_euclidian_distance_01() {
        assert_eq!(compute_euclidian_distance(&(1, 1, 1), &(2, 2, 2)), 3);
    }
}

#[allow(clippy::too_many_lines)]
fn fold(junction_box_list: &Vec<Coords>, connection_count: usize) -> usize {
    let mut nearest_list: Vec<(Coords, Coords, i64)> =
        Vec::with_capacity(junction_box_list.len().pow(2) / 2);

    // println!("Start");
    for i in 0..junction_box_list.len() {
        for j in (i + 1)..junction_box_list.len() {
            nearest_list.push((
                junction_box_list[i],
                junction_box_list[j],
                compute_euclidian_distance(&junction_box_list[i], &junction_box_list[j]),
            ));
        }
    }
    // println!("End");
    let mut nearest_list_sorted_vec: Vec<((Coords, Coords), i64)> =
        nearest_list.iter().map(|v| ((v.0, v.1), v.2)).collect();
    nearest_list_sorted_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("End Sort");

    // println!("Nearest list: {nearest_list:?}\n");
    // println!();
    // println!("Nearest list length: {}\n", nearest_list.len());

    // println!("Nearest list, sorted in ascending distance");
    // for (couple, distance) in &nearest_list_sorted_vec {
        // println!("{distance} --> {couple:?}");
    // }
    // println!();
    // println!(
        // "Nearest list, sorted, length: {}\n",
        // nearest_list_sorted_vec.len()
    // );

    let mut circuit_list: Vec<BTreeSet<Coords>> = vec![];

    let mut tmp_connection_count = 0;
    for ((junction_box, other_box), _) in nearest_list_sorted_vec {
        // println!("tmp_connection_count: {tmp_connection_count}");
        // println!("Assembling {junction_box:?} and {other_box:?}");
        let tmp_circuit_list = circuit_list.clone();
        let option_junction_box = tmp_circuit_list
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(&junction_box));
        let option_other_box = tmp_circuit_list
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(&other_box));

        if tmp_connection_count < connection_count {
            match (option_junction_box, option_other_box) {
                (Some((_, c1)), Some((_, c2))) if c1 == c2 => {
                    // println!(
                        // "Already in the same circuit (len {} and len {}), nothing to do.",
                        // c1.len(),
                        // c2.len()
                    // );
                    tmp_connection_count += 1;
                }
                (Some((index_junction_box_circuit, c1)), Some((index_other_box_circuit, c2)))
                    if c1 != c2 =>
                {
                    // println!(
                        // "Not in the same circuits (len {} and len {}), merging circuits",
                        // c1.len(),
                        // c2.len()
                    // );
                    let new_circuit = c1.union(c2).copied().collect::<BTreeSet<Coords>>();
                    if index_junction_box_circuit < index_other_box_circuit {
                        circuit_list.swap_remove(index_other_box_circuit);
                        circuit_list.swap_remove(index_junction_box_circuit);
                    } else {
                        circuit_list.swap_remove(index_junction_box_circuit);
                        circuit_list.swap_remove(index_other_box_circuit);
                    }
                    circuit_list.push(new_circuit.clone());
                    // println!(
                        // "Merged circuit: {new_circuit:?}. Its length: {}",
                        // new_circuit.len()
                    // );
                    tmp_connection_count += 1;
                }
                (Some(_), Some(_)) => {
                    panic!("WHAT THE F ?!?!");
                }
                (Some(circuit_junction_box), None) => {
                    // println!(
                        // "Found circuit with {circuit_junction_box:?}. Its length: {}",
                        // circuit_junction_box.1.len()
                    // );
                    let mut_set = circuit_list
                        .iter_mut()
                        .find(|circuit| circuit.contains(&junction_box))
                        .unwrap();
                    mut_set.insert(other_box);
                    // println!(
                        // "Updated circuit: {mut_set:?}. Its new length: {}",
                        // mut_set.len()
                    // );
                    tmp_connection_count += 1;
                }
                (None, Some(circuit_other_box)) => {
                    // println!(
                        // "Found circuit with {circuit_other_box:?}. Its length: {}",
                        // circuit_other_box.1.len()
                    // );
                    let mut_set = circuit_list
                        .iter_mut()
                        .find(|circuit| circuit.contains(&other_box))
                        .unwrap();
                    mut_set.insert(junction_box);
                    // println!(
                        // "Updated circuit: {mut_set:?}. Its new length: {}",
                        // mut_set.len()
                    // );
                    tmp_connection_count += 1;
                }
                (None, None) => {
                    let new_circuit = BTreeSet::from_iter(vec![junction_box, other_box]);
                    // println!("Didn't find a circuit. Creating one: {new_circuit:?}");
                    circuit_list.push(new_circuit);
                    tmp_connection_count += 1;
                }
            }
        } else {
            break;
        }
    }

    // println!("Connection count: {tmp_connection_count}");

    // println!();
    // println!("Circuit list");
    // for circuit in &circuit_list {
    // println!("--> {circuit:?}");
    // }
    // println!();

    let mut circuit_list_sorted_descending: Vec<(BTreeSet<Coords>, usize)> = vec![];

    for circuit in circuit_list {
        let mut inserted = false;
        let circuit_length = circuit.len();
        let tmp_circuit_list_sorted_descending = circuit_list_sorted_descending.clone();
        for (index, (_, sorted_length)) in tmp_circuit_list_sorted_descending.iter().enumerate() {
            if circuit_length > *sorted_length {
                inserted = true;
                circuit_list_sorted_descending.insert(index, (circuit.clone(), circuit_length));
                break;
            }
        }
        if !inserted {
            circuit_list_sorted_descending.push((circuit, circuit_length));
        }
    }

    // println!("Circuit list, sorted in descending order");
    // for (circuit, length) in &circuit_list_sorted_descending {
        // println!("{length} --> {circuit:?}");
        // println!();
    // }

    // println!(
        // "Total boxes: {}",
        // circuit_list_sorted_descending
            // .iter()
            // .map(|circuit| circuit.0.len() as u64)
            // .sum::<u64>()
    // );

    // println!("Circuit count: {}", circuit_list_sorted_descending.len());

    let result = circuit_list_sorted_descending
        .iter()
        .take(3)
        .map(|(_, circuit_length)| circuit_length)
        .product();
    // println!("RESULT: {result}");
    result
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_sample() {
        let coord_list = parse_content(
            "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!(fold(&coord_list, 10), 40);
    }

    #[test]
    fn fold_final() {
        let coord_list = parse_content(&get_file_content("assets/input"));
        assert_eq!(fold(&coord_list, 1000), 0);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let coord_list = parse_content(&file_content);
    println!("Result: {}", fold(&coord_list, 1000));
}

// 18252 is too low
