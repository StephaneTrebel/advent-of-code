use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(content: &str) {
    let mut count = 0;
    for line in content.lines() {
        if line.contains('x') {
            let mut split = line.split(": ");
            let mut region = split.next().expect("Region should exist").split('x');
            let a = region
                .next()
                .expect("A should exist")
                .parse::<usize>()
                .expect("A should be an integer");
            let b = region
                .next()
                .expect("B should exist")
                .parse::<usize>()
                .expect("B should be an integer");

            let present_area: usize = split
                .next()
                .expect("presents should be there")
                .split_whitespace()
                .enumerate()
                .map(|(i, p)| {
                    let parsed_p = p.parse::<usize>().expect("parsed_p is not an integer");
                    match i {
                        3 => parsed_p * 6,
                        0 | 1 | 2 | 4 => parsed_p * 7,
                        _ => parsed_p * 5,
                    }
                })
                .sum();
            let diff: isize = (a * b) as isize - present_area as isize;
            if diff >= 0 {
                count += 1;
                println!(
                    "A:{a}, B:{b}, AxB:{} Present area: {present_area} Sub {}",
                    a * b,
                    diff
                );
            }
        }
    }
    println!("Count: {count}");
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    parse_content(&file_content);
    // println!("Result: {}", fold(graph));
}
