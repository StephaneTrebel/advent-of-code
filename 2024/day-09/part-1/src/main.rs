use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BlockType {
    FreeSpace,
    File(usize),
}

type DiskMap = Vec<BlockType>;

#[derive(Debug, PartialEq, Clone)]
struct Content {
    disk_map: DiskMap,
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        let mut disk_map: DiskMap = vec![];
        let mut file_id: usize = 0;

        for (id, block_count_char) in value.chars().enumerate() {
            if block_count_char.is_ascii_digit() {
                let block_count = block_count_char
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("Char '{}' should be a number", block_count_char))
                    as usize;
                if id % 2 == 0 {
                    (0..block_count).for_each(|_| disk_map.push(BlockType::File(file_id)));
                    file_id += 1;
                } else {
                    (0..block_count)
                        .map(|_| '.')
                        .for_each(|_| disk_map.push(BlockType::FreeSpace));
                }
            }
        }

        Self { disk_map }
    }
}

#[cfg(test)]
mod tests_content {
    use super::*;

    #[test]
    fn content_from_simple() {
        let content = Content::from("12345");
        assert_eq!(
            content,
            Content {
                disk_map: vec![
                    BlockType::File(0),
                    BlockType::FreeSpace,
                    BlockType::FreeSpace,
                    BlockType::File(1),
                    BlockType::File(1),
                    BlockType::File(1),
                    BlockType::FreeSpace,
                    BlockType::FreeSpace,
                    BlockType::FreeSpace,
                    BlockType::FreeSpace,
                    BlockType::File(2),
                    BlockType::File(2),
                    BlockType::File(2),
                    BlockType::File(2),
                    BlockType::File(2)
                ]
            }
        );
    }
    #[test]
    fn content_from_less_simple() {
        let content = Content::from("2333133121414131402");
        assert_eq!(
            content,
            Content {
                disk_map: vec![
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
                    BlockType::File(9)
                ]
            }
        );
    }
}

fn move_file_blocks(disk_map: &mut DiskMap) {
    let mut pos = 0;
    while let Some(&value) = disk_map.get(pos) {
        match value {
            BlockType::FreeSpace => {
                let mut rev_pos: isize = (disk_map.len() - 1) as isize;
                while let Some(&rev_value) = disk_map.get(rev_pos as usize) {
                    if rev_value != BlockType::FreeSpace && rev_pos > pos as isize {
                        // println!("Swapping {} and {}", pos, rev_pos);
                        disk_map.swap(pos, rev_pos as usize);
                        break;
                    }
                    rev_pos -= 1;
                }
            }
            BlockType::File(_) => {}
        }
        pos += 1;
    }
}

#[cfg(test)]
mod tests_move_file_blocks {
    use super::*;

    #[test]
    fn move_file_blocks_01() {
        let mut content = Content::from("12345");
        move_file_blocks(&mut content.disk_map);
        assert_eq!(
            content.disk_map,
            vec![
                BlockType::File(0),
                BlockType::File(2),
                BlockType::File(2),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(2),
                BlockType::File(2),
                BlockType::File(2),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace
            ]
        );
    }

    #[test]
    fn move_file_blocks_02() {
        let mut content = Content::from("2333133121414131402");
        move_file_blocks(&mut content.disk_map);
        assert_eq!(
            content.disk_map,
            vec![
                BlockType::File(0),
                BlockType::File(0),
                BlockType::File(9),
                BlockType::File(9),
                BlockType::File(8),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(1),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(8),
                BlockType::File(2),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::File(7),
                BlockType::File(3),
                BlockType::File(3),
                BlockType::File(3),
                BlockType::File(6),
                BlockType::File(4),
                BlockType::File(4),
                BlockType::File(6),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(5),
                BlockType::File(6),
                BlockType::File(6),
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace,
                BlockType::FreeSpace
            ]
        );
    }
}

fn compute_checksum(disk_map: &DiskMap) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(|(pos, &c)| match c {
            BlockType::FreeSpace => 0,
            BlockType::File(f) => pos * f,
        })
        .sum()
}

#[cfg(test)]
mod tests_compute_checksum {
    use super::*;

    #[test]
    fn compute_checksum_01() {
        let mut content = Content::from("2333133121414131402");
        move_file_blocks(&mut content.disk_map);
        assert_eq!(compute_checksum(&content.disk_map,), 1928);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let mut content = Content::from(get_file_content("assets/input").as_str());

    move_file_blocks(&mut content.disk_map);
    println!("Result: {}", compute_checksum(&content.disk_map));
}
