use std::{fmt::Display, fs, slice::Iter};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
enum BlockType {
    FreeSpace,
    File(usize),
}

#[derive(Debug, PartialEq, Clone)]
struct DiskMap(Vec<BlockType>);

impl From<Vec<BlockType>> for DiskMap {
    fn from(value: Vec<BlockType>) -> Self {
        Self(value)
    }
}

impl DiskMap {
    fn get(&self, &position: &usize) -> BlockType {
        match self.0.get(position) {
            Some(block) => block.clone(),
            None => BlockType::FreeSpace,
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn swap_blocks(&mut self, &from: &usize, &to: &usize) {
        self.0.swap(from, to);
    }

    fn iter(&self) -> Iter<'_, BlockType> {
        self.0.iter()
    }

    fn push(&mut self, file_id: BlockType) {
        self.0.push(file_id);
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let mut disk_map: DiskMap = DiskMap::from(vec![]);
        let mut file_id: usize = 0;

        for (position, block_count_char) in value.chars().enumerate() {
            if block_count_char.is_ascii_digit() {
                let _block_count = block_count_char
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("Char '{}' should be a number", block_count_char))
                    as usize;

                if position % 2 == 0 {
                    for _ in 0.._block_count {
                        disk_map.push(BlockType::File(file_id));
                    }
                    file_id += 1;
                } else {
                    for _ in 0.._block_count {
                        disk_map.push(BlockType::FreeSpace);
                    }
                }
            }
        }

        disk_map
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut temp: Vec<char> = vec![];
            let mut i = 0;
            while i < self.len() {
                let block_type = self.get(&i);
                match block_type {
                    BlockType::FreeSpace => {
                        temp.push('.');
                        i += 1
                    }
                    BlockType::File(id) => {
                        temp.push(char::from_digit(id as u32, 10).unwrap_or('Z'));
                        i += 1;
                    }
                }
            }
            temp.iter().collect::<String>()
        })
    }
}

#[cfg(test)]
mod tests_disk_map {
    use super::*;

    #[test]
    fn disk_map_from_simple() {
        let disk_map = DiskMap::from("12345");
        pretty_assertions::assert_eq!(
            disk_map,
            DiskMap::from(vec![
                BlockType::File(0,),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(1,),
                BlockType::File(1,),
                BlockType::File(1,),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(2,),
                BlockType::File(2,),
                BlockType::File(2,),
                BlockType::File(2,),
                BlockType::File(2,),
            ])
        );
        pretty_assertions::assert_eq!(format!("{}", disk_map), "0..111....22222");
    }

    #[test]
    fn disk_map_from_less_simple() {
        let disk_map = DiskMap::from("2333133121414131402");
        pretty_assertions::assert_eq!(
            disk_map,
            DiskMap::from(vec![
                BlockType::File(0),
                BlockType::File(0),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(2),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(3),
                BlockType::File(3),
                BlockType::File(3),
                BlockType::FreeSpace,
                BlockType::File(4),
                BlockType::File(4),
                BlockType::FreeSpace,
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::FreeSpace,
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::FreeSpace,
                BlockType::File(7),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::FreeSpace,
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(9),
                BlockType::File(9),
            ])
        );
        pretty_assertions::assert_eq!(
            format!("{}", disk_map),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }
}

fn move_file_blocks(disk_map: &mut DiskMap) {
    // println!("Layout: {}", disk_map);
    let mut last_moved_id = usize::MAX;
    let disk_length = disk_map.len();

    // Cursor will move backwards to get file by ID descending order
    let mut cursor = disk_length - 1;
    let mut temp_id = usize::MAX;
    while cursor > 0 {
        let block_type = disk_map.get(&cursor);
        if let BlockType::File(id) = block_type {
            if id != temp_id {
                temp_id = id;
                // println!("BlockType starting at {} is a File with id {}", cursor, id);
                if id < last_moved_id {
                    // Getting file length
                    let mut file_length = 0;
                    let mut file_cursor = cursor;
                    let mut file_type = block_type;
                    while file_type == BlockType::File(id) && file_cursor > 0 {
                        file_length += 1;
                        file_cursor -= 1;
                        file_type = disk_map.get(&file_cursor);
                    }

                    // println!(
                    // "Attempting to find a span of free space of length >= {}",
                    // file_length
                    // );
                    let mut rev_cursor = 0;
                    while rev_cursor < disk_length && rev_cursor < cursor {
                        let rev_block_type = disk_map.get(&rev_cursor);
                        if let BlockType::FreeSpace = rev_block_type {
                            // println!(
                            // "FreeSpace found at position {}. Let's see if File {} fits",
                            // rev_cursor, id
                            // );

                            let mut temp_pos = rev_cursor;
                            let mut freespace_length = 0;
                            while let BlockType::FreeSpace = disk_map.get(&temp_pos) {
                                freespace_length += 1;
                                if temp_pos < disk_length {
                                    temp_pos += 1;
                                } else {
                                    break;
                                }
                            }

                            // println!("FreeSpace span length is {}", freespace_length);
                            if freespace_length >= file_length {
                                // println!("Span is large enough. Moving file blocks…");
                                for i in 0..file_length {
                                    let p = cursor - i;
                                    let r = rev_cursor + i;
                                    // println!("Swapping blocks {} and {}", p, r);
                                    disk_map.swap_blocks(&p, &r);
                                    // println!("Layout: {}", disk_map);
                                }
                                cursor -= file_length - 1;
                                last_moved_id = id;
                                break;
                            } else {
                                // println!("Not suitable");
                            }
                        }
                        rev_cursor += 1;
                    }
                }
            }
        }
        cursor -= 1;
    }
}

#[cfg(test)]
mod tests_move_file_blocks {
    use super::*;

    #[test]
    fn move_file_blocks_final_result() {
        let mut disk_map = DiskMap::from("2333133121414131402");
        move_file_blocks(&mut disk_map);
        pretty_assertions::assert_eq!(
            disk_map,
            DiskMap::from(vec![
                BlockType::File(0),
                BlockType::File(0),
                BlockType::File(9),
                BlockType::File(9),
                BlockType::File(2),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::FreeSpace,
                BlockType::File(4),
                BlockType::File(4),
                BlockType::FreeSpace,
                BlockType::File(3),
                BlockType::File(3),
                BlockType::File(3),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::FreeSpace,
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
            ])
        );
        pretty_assertions::assert_eq!(
            format!("{}", disk_map),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }
}

fn compute_checksum(disk_map: &DiskMap) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(move |(i, block_type)| match block_type {
            BlockType::FreeSpace => 0,
            BlockType::File(id) => i * id,
        })
        .sum()
}

#[cfg(test)]
mod tests_compute_checksum {
    use super::*;

    #[test]
    fn compute_checksum_01() {
        pretty_assertions::assert_eq!(
            compute_checksum(&DiskMap::from(vec![
                BlockType::File(0),
                BlockType::File(0),
                BlockType::File(9),
                BlockType::File(9),
                BlockType::File(2),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::FreeSpace,
                BlockType::File(4),
                BlockType::File(4),
                BlockType::FreeSpace,
                BlockType::File(3),
                BlockType::File(3),
                BlockType::File(3),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::FreeSpace,
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
            ])),
            2858
        );
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let mut disk_map = DiskMap::from(get_file_content("assets/input").as_str());

    move_file_blocks(&mut disk_map);

    println!("Result: {}", compute_checksum(&disk_map));
}
