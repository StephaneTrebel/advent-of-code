fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    std::fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Clone, PartialEq)]
struct Button {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Coords {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Prize {
    a_button: Button,
    b_button: Button,
    target_coordinates: Coords,
}

fn parse_content(content: &str) -> Vec<Prize> {
    let mut prize_list: Vec<Prize> = vec![];
    let mut a_button: Option<Button> = None;
    let mut b_button: Option<Button> = None;
    for (index, line) in content.lines().enumerate() {
        match index % 4 {
            0 => {
                // Button A
                let splitted = line.split(':').nth(1).expect("No right side");
                let mut comma_split = splitted.split(',');

                let x = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('+')
                    .nth(1)
                    .expect("No X increment on button A")
                    .parse::<f64>()
                    .expect("Cannot parse X increment on button A");

                let y = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('+')
                    .nth(1)
                    .expect("No Y increment on button A")
                    .parse::<f64>()
                    .expect("Cannot parse Y increment on button A");

                a_button = Some(Button { x, y });
            }
            1 => {
                // Button B
                let splitted = line.split(':').nth(1).expect("No right side");
                let mut comma_split = splitted.split(',');

                let x_increment = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('+')
                    .nth(1)
                    .expect("No X increment on button B")
                    .parse::<f64>()
                    .expect("Cannot parse X increment on button B");

                let y_increment = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('+')
                    .nth(1)
                    .expect("No Y increment on button B")
                    .parse::<f64>()
                    .expect("Cannot parse Y increment on button B");

                b_button = Some(Button {
                    x: x_increment,
                    y: y_increment,
                });
            }
            2 => {
                // Prize
                let splitted = line.split(':').nth(1).expect("No right side");
                let mut comma_split = splitted.split(',');

                let x = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('=')
                    .nth(1)
                    .expect("No X on prize")
                    .parse::<f64>()
                    .expect("Cannot parse X coordinate on button B");

                let y = comma_split
                    .next()
                    .expect("No left side of comma")
                    .split('=')
                    .nth(1)
                    .expect("No Y on prize")
                    .parse::<f64>()
                    .expect("Cannot parse Y coordinate on button B");

                let new_prize = Prize {
                    a_button: a_button.clone().expect("A button should be defined"),
                    b_button: b_button.clone().expect("B button should be defined"),
                    target_coordinates: Coords { x, y },
                };
                // println!("New prize: {new_prize:?}");
                prize_list.push(new_prize);
            }

            _ => {}
        }
    }

    prize_list
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_sample() {
        let prize_list = parse_content(
            "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );

        assert_eq!(
            prize_list,
            vec![
                Prize {
                    a_button: Button { x: 94., y: 34. },
                    b_button: Button { x: 22., y: 67. },
                    target_coordinates: Coords { x: 8400., y: 5400. }
                },
                Prize {
                    a_button: Button { x: 26., y: 66. },
                    b_button: Button { x: 67., y: 21. },
                    target_coordinates: Coords {
                        x: 12748.,
                        y: 12176.
                    }
                },
                Prize {
                    a_button: Button { x: 17., y: 86. },
                    b_button: Button { x: 84., y: 37. },
                    target_coordinates: Coords { x: 7870., y: 6450. }
                },
                Prize {
                    a_button: Button { x: 69., y: 23. },
                    b_button: Button { x: 27., y: 71. },
                    target_coordinates: Coords {
                        x: 18641.,
                        y: 10279.
                    }
                }
            ]
        );
    }
}

const OFFSET: f64 = 10_000_000_000_000.;

fn get_mininum_tokens_to_win(
    Prize {
        a_button,
        b_button,
        target_coordinates,
    }: &Prize,
) -> f64 {
    let mut tokens = 0.;
    // println!("Prize: {target_coordinates:?}");
    let tmp_x = target_coordinates.x + OFFSET;
    let tmp_y = target_coordinates.y + OFFSET;
    // println!("Offset coordinates: {:?}", (tmp_x, tmp_y));
    let determinant: f64 = (a_button.x * b_button.y) - (b_button.x * a_button.y);
    // println!("Determinant: {determinant}");
    if determinant != 0. {
        let tmp_a = tmp_x * b_button.y - b_button.x * tmp_y;
        // println!("tmp_a: {tmp_a}");
        let a = tmp_a / determinant;
        let tmp_b = a_button.x * tmp_y - a_button.y * tmp_x;
        // println!("tmp_b: {tmp_b}");
        let b = tmp_b / determinant;

        // println!("A: {a}, B: {b}");

        if a >= 0. && b >= 0. && a.fract() == 0. && b.fract() == 0. {
            tokens = a * 3. + b;
        }
    }

    // println!("Tokens {tokens}");
    tokens
}

#[cfg(test)]
mod tests_get_mininum_tokens_to_win {
    use super::*;

    #[test]
    fn get_mininum_tokens_to_win_0() {
        let prize_list = parse_content(
            "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        );

        assert_eq!(get_mininum_tokens_to_win(&prize_list[0]), 0.);
    }

    #[test]
    fn get_mininum_tokens_to_win_1() {
        let prize_list = parse_content(
            "\
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176",
        );

        assert_eq!(get_mininum_tokens_to_win(&prize_list[0]), 459236326669.);
    }

    #[test]
    fn get_mininum_tokens_to_win_2() {
        let prize_list = parse_content(
            "\
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450",
        );

        assert_eq!(get_mininum_tokens_to_win(&prize_list[0]), 0.);
    }

    #[test]
    fn get_mininum_tokens_to_win_3() {
        let prize_list = parse_content(
            "\
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );

        assert_eq!(get_mininum_tokens_to_win(&prize_list[0]), 416082282239.);
    }
}

fn fold(prize_list: &[Prize]) -> f64 {
    prize_list.iter().map(get_mininum_tokens_to_win).sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    println!("Result: {}", fold(&parse_content(&file_content)));
}

// 31237
// 31580
