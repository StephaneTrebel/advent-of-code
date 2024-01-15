use std::fs;

// use itertools::Itertools;
// use rayon::prelude::*;
use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type CoordType = num::rational::Ratio<i128>;

type Hail = [[CoordType; 3]; 2];

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
                let x: CoordType = captures["px"].parse().unwrap();
                let y: CoordType = captures["py"].parse().unwrap();
                let z: CoordType = captures["pz"].parse().unwrap();
                let vx: CoordType = captures["vx"].parse().unwrap();
                let vy: CoordType = captures["vy"].parse().unwrap();
                let vz: CoordType = captures["vz"].parse().unwrap();
                [[x, y, z], [vx, vy, vz]]
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
        assert_eq!(content.list.get(3).is_some(), true);
    }
}

// Math time !
//
// Be R = "xr yr zr @ vxr vyr vzr" the Rock we will throw, with its coordinates and its starting velocity
// Be H1= "x1 y1 z1 @ vx1 vy1 vz1", H2 (same idea) and H3 (same idea) three Hailstones.
//
// Assumptions:
// - We KNOW that R WILL hit H1, H2, and H3, albeit at different times
// - We KNOW all x/y/z/vx/vx/vz values for H1, H2, and H3
// - xr/yr/zr are unsigned integers, and vxr/vyr/vzr are signed integers
//
// Equations:
// xr(t) = xr + t * vxr (same for yr and zr)
// x1(t) = x1 + t * vx1 (same for y1 and z1, and same for H2 and H3)
//
// So, based on the Assumptions and Equations, that there exist a positive integer t1 that satisfies:
//
// xr(t1) = x1(t1)
// yr(t1) = y1(t1)
// zr(t1) = z1(t1)
//
// xr + t1 * vxr = x1 + t1 * vx1
// yr + t1 * vyr = y1 + t1 * vy1
// zr + t1 * vzr = z1 + t1 * vz1
//
// And for H2 and H3:
//
// xr(t2) = x2(t2)
// yr(t2) = y2(t2)
// zr(t2) = z2(t2)
//
// xr + t2 * vxr = x2 + t2 * vx2
// yr + t2 * vyr = y2 + t2 * vy2
// zr + t2 * vzr = z2 + t2 * vz2
//
// xr(t3) = x3(t3)
// yr(t3) = y3(t3)
// zr(t3) = z3(t3)
//
// xr + t3 * vxr = x3 + t3 * vx3
// yr + t3 * vyr = y3 + t3 * vy3
// zr + t3 * vzr = z3 + t3 * vz3
//
//
// Simplifying the first one, regarding H1:
//
// t1 = - (xr - x1) / (vxr - vx1)
// t1 = - (yr - y1) / (vyr - vy1)
// t1 = - (zr - z1) / (vzr - vz1)
//
// So with a bit of shuffling around these three equivalent lines (all equal to t1)
// and also apply them to H2, we can get:
// (vzr-vz1)/(zr-z1) = ( (vy1-v2y)(xr-x2)+(vx2-vx1)(yr-y2) ) / ( (xr-x1)(yr-y2)-(yr-y1)(xr-x2) )
//
// More simply: (vzr-vz1)/(zr-z1) = A / B
// where A = (vy1-v2y)(xr-x2) + (vx2-vx1)(yr-y2)
// and B = (xr-x1)(yr-y2) - (yr-y1)(xr-x2)
//
// Expanding all the terms and letting some of them cancel eventually gives us:
// (V1-V2)×(P1-P2)⋅P = (V1-V2)⋅P1×P2
// Where:
// - V1 and V2 are the velocitiy matrices of H1 and H2, respectively
// - P1 and P2 are the position matrices of H1 and H2, respectively
// - P is a scalar of Rock positions (x,y,z)
//
// Rotating around with H1, H2, and H3 to get more equations yield:
// (V1-V2)×(P1-P2)⋅P = (V1-V2)⋅P1×P2
// (V1-V3)×(P1-P3)⋅P = (V1-V3)⋅P1×P3
// (V2-V3)×(P2-P3)⋅P = (V2-V3)⋅P2×P3
//
// And now you just have to solve for P (everything else is known)

fn part2(content: &Content) -> CoordType {
    // Set up equation matrix
    let [[x1, y1, z1], [vx1, vy1, vz1]] = content.list[0];
    let [[x2, y2, z2], [vx2, vy2, vz2]] = content.list[1];
    let [[x3, y3, z3], [vx3, vy3, vz3]] = content.list[2];

    // This solver is way nicer that mine lol
    // Thanks to https://github.com/LinAGKar/advent-of-code-2023-rust
    let mut equations = [
        [
            CoordType::default(),
            vz1 - vz3,
            vy3 - vy1,
            CoordType::default(),
            z3 - z1,
            y1 - y3,
            y1 * vz1 - z1 * vy1 - y3 * vz3 + z3 * vy3,
        ],
        [
            vz1 - vz3,
            CoordType::default(),
            vx3 - vx1,
            z3 - z1,
            CoordType::default(),
            x1 - x3,
            x1 * vz1 - z1 * vx1 - x3 * vz3 + z3 * vx3,
        ],
        [
            vy3 - vy1,
            vx1 - vx3,
            CoordType::default(),
            y1 - y3,
            x3 - x1,
            CoordType::default(),
            y1 * vx1 - x1 * vy1 - y3 * vx3 + x3 * vy3,
        ],
        [
            CoordType::default(),
            vz2 - vz3,
            vy3 - vy2,
            CoordType::default(),
            z3 - z2,
            y2 - y3,
            y2 * vz2 - z2 * vy2 - y3 * vz3 + z3 * vy3,
        ],
        [
            vz2 - vz3,
            CoordType::default(),
            vx3 - vx2,
            z3 - z2,
            CoordType::default(),
            x2 - x3,
            x2 * vz2 - z2 * vx2 - x3 * vz3 + z3 * vx3,
        ],
        [
            vy3 - vy2,
            vx2 - vx3,
            CoordType::default(),
            y2 - y3,
            x3 - x2,
            CoordType::default(),
            y2 * vx2 - x2 * vy2 - y3 * vx3 + x3 * vy3,
        ],
    ];

    // Perform gaussian elimination
    // Iterate diagonally from top left, to turn matrix into reduced row echelon form
    for i in 0..6 {
        // Find non-zero item in current column, from current row or after
        let non_zero_row = (i..6)
            .find(|&row| equations[row][i] != CoordType::default())
            .unwrap();

        // Swap current row with first non-zero row
        if non_zero_row != i {
            (equations[i], equations[non_zero_row]) = (equations[non_zero_row], equations[i]);
        }

        // Divide row by value at current pos, to turn value into 1
        let curr_val = equations[i][i];
        equations[i][i] = CoordType::from_integer(1);
        for item in &mut equations[i][i + 1..] {
            *item /= curr_val;
        }

        // Subtract multiple of current row from lower rows, to turn column below current item to 0
        for row in i + 1..6 {
            let multiple = equations[row][i];
            equations[row][i] = CoordType::default();
            if multiple != CoordType::default() {
                for col in i + 1..7 {
                    equations[row][col] -= equations[i][col] * multiple;
                }
            }
        }
    }

    // Iterate diagonally from bottom right, to turn matrix (except last column) into unit matrix.
    for i in (0..6).rev() {
        for row in 0..i {
            equations[row][6] -= equations[i][6] * equations[row][i];
            equations[row][i] = CoordType::default();
        }
    }

    equations.iter().take(3).map(|x| x[6]).sum::<CoordType>()
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Part 2: {}", part2(&content));
}
