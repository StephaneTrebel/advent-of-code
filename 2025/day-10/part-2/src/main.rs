use std::fs;

use good_lp::{
    Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, default_solver,
    variable,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
struct Button(Vec<u32>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
struct JoltageList(Vec<u32>);

impl std::fmt::Display for JoltageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = self
            .0
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        write!(f, "{{{}}}", temp.join(","))
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = self
            .0
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        write!(f, "{{{}}}", temp.join(","))
    }
}

#[derive(Debug, PartialEq)]
struct Machine {
    button_list: Vec<Button>,
    joltage_requirements: JoltageList,
}

impl TryFrom<&str> for Machine {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        println!("Parsing {line}");
        let mut split = line.split_whitespace();

        let indicator_light_diagram: Vec<bool> = {
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
            list
        };
        let zero: Vec<u32> = indicator_light_diagram.clone().iter().map(|_| 0).collect();

        let mut last_block = "";
        let button_wiring_schematic_list: Vec<Button> = {
            let mut list: Vec<Button> = vec![];
            for block in split {
                match block {
                    s if s.contains('{') => {
                        last_block = s;
                        break;
                    }
                    s => {
                        list.push({
                            let mut tmp_button_list = zero.clone();
                            for c in s.chars() {
                                match c {
                                    '(' | ')' | ',' => {}
                                    integer => {
                                        tmp_button_list[integer
                                            .to_digit(10)
                                            .unwrap_or_else(|| panic!("Not an integer {integer}"))
                                            as usize] = 1;
                                    }
                                }
                            }
                            Button(tmp_button_list)
                        });
                    }
                }
            }
            list
        };

        let joltage_requirements: JoltageList = {
            let mut tmp_joltage_requirements_list: JoltageList = JoltageList(vec![]);
            let last_block_cleaned = last_block.replace(['{', '}'], "");

            for integer in last_block_cleaned.split(',') {
                tmp_joltage_requirements_list.0.push(
                    integer
                        .parse()
                        .unwrap_or_else(|i| panic!("Not an integer {i}")),
                );
            }
            tmp_joltage_requirements_list
        };

        Ok(Self {
            button_list: button_wiring_schematic_list,
            joltage_requirements,
        })
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{{}}}",
            self.button_list
                .iter()
                .map(|x| format!(
                    "({})",
                    x.0.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(",")
                ))
                .collect::<Vec<String>>()
                .join(" "),
            self.joltage_requirements
                .0
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_parse_machine_to_string {
    use super::*;

    #[test]
    fn parse_machine_to_string_01() {
        let machine =
            Machine::try_from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(
            machine.to_string(),
            "(0,0,0,1) (0,1,0,1) (0,0,1,0) (0,0,1,1) (1,0,1,0) (1,1,0,0) {3,5,4,7}"
        );
    }

    #[test]
    fn parse_machine_to_string_02() {
        let machine =
            Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        assert_eq!(
            machine.to_string(),
            "(1,0,1,1,1) (0,0,1,1,0) (1,0,0,0,1) (1,1,1,0,0) (0,1,1,1,1) {7,5,12,7,2}"
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
[.##.] (2,3) (1,3) (3) (0,2) (2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
",
        );
        assert_eq!(
            machine_list,
            vec![
                Machine {
                    button_list: vec![
                        Button(vec![0, 0, 1, 1]), //n->4
                        Button(vec![0, 1, 0, 1]), //p->5
                        Button(vec![0, 0, 0, 1]), //q->7
                        Button(vec![1, 0, 1, 0]), //r->3
                        Button(vec![0, 0, 1, 0]), //s->4
                        Button(vec![1, 1, 0, 0])  //t->3
                    ],
                    joltage_requirements: JoltageList(vec![3, 5, 4, 7])
                },
                Machine {
                    button_list: vec![
                        Button(vec![1, 0, 1, 1, 1]),
                        Button(vec![0, 0, 1, 1, 0]),
                        Button(vec![1, 0, 0, 0, 1]),
                        Button(vec![1, 1, 1, 0, 0]),
                        Button(vec![0, 1, 1, 1, 1])
                    ],
                    joltage_requirements: JoltageList(vec![7, 5, 12, 7, 2])
                },
                Machine {
                    button_list: vec![
                        Button(vec![1, 1, 1, 1, 1, 0]),
                        Button(vec![1, 0, 0, 1, 1, 0]),
                        Button(vec![1, 1, 1, 0, 1, 1]),
                        Button(vec![0, 1, 1, 0, 0, 0])
                    ],
                    joltage_requirements: JoltageList(vec![10, 11, 11, 5, 10, 5])
                }
            ]
        );
    }
}

struct MachineProblem {
    problem_variables: ProblemVariables,
    variable_definitions: Vec<Variable>,
    expressions: Vec<Expression>,
    joltage_requirements: JoltageList,
    objective: Expression,
}

impl From<&Machine> for MachineProblem {
    fn from(machine: &Machine) -> Self {
        let mut new_machine_problem = MachineProblem::new(machine.joltage_requirements.clone());

        for (button_index, _) in machine.button_list.iter().enumerate().rev() {
            let new_variable =
                new_machine_problem.add_variable(&machine.button_list.clone(), button_index);
            new_machine_problem.variable_definitions.push(new_variable);
        }
        new_machine_problem
    }
}

impl MachineProblem {
    fn new(joltage_requirements: JoltageList) -> Self {
        let joltage_count = joltage_requirements.0.len();
        Self {
            problem_variables: ProblemVariables::new(),
            variable_definitions: vec![],
            expressions: (0..joltage_count)
                .map(|_| Expression::with_capacity(joltage_count))
                .collect(),
            joltage_requirements,
            objective: Expression::with_capacity(joltage_count),
        }
    }

    fn add_variable(&mut self, button_list: &[Button], button_index: usize) -> Variable {
        let new_variable = self.problem_variables.add(variable().integer().min(0).initial(0));
        for (joltage_index, expression) in self.expressions.iter_mut().enumerate() {
            if button_list[button_index].0[joltage_index] == 1 {
                *expression += new_variable;
            }
        }
        self.objective += new_variable;
        new_variable
    }

    #[allow(clippy::cast_lossless)]
    fn get_constraints(&self) -> Vec<Constraint> {
        self.expressions
            .iter()
            .enumerate()
            .map(|(index, expression)| {
                expression
                    .clone()
                    .eq(self.joltage_requirements.0[index] as f64)
            })
            .collect()
    }
}

impl Machine {
    fn calc_min_buttons(&self) -> u32 {
        println!("Machine: {self}");
        println!("Solving with CBC algorithm...");

        let machine_problem: MachineProblem = self.into();

        let solution = machine_problem
            .problem_variables
            .clone()
            .minimise(machine_problem.objective.clone())
            .using(default_solver) // IBM's coin_cbc by default
            .with_all(machine_problem.get_constraints())
            .solve()
            .expect("Should work");

        println!("Variables:");
        for (index, variable_definition) in machine_problem.problem_variables.into_iter().enumerate() {
            println!(
                "{:?}={}",
                variable_definition,
                solution.value(machine_problem.variable_definitions[index])
            );
        }

        println!("Expressions:");
        for (index, expression) in machine_problem.expressions.into_iter().enumerate() {
            println!("{index}={expression:?}");
        }

        println!("Objective: {:?}", machine_problem.objective);

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let min: u32 = solution.eval(machine_problem.objective) as u32;
        println!("Min = {min}");

        min
    }
}

#[cfg(test)]
mod tests_calc_min_buttons {
    use super::*;

    #[test]
    fn calc_min_buttons_machine_01() {
        let machine =
            Machine::try_from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(machine.calc_min_buttons(), 10);
    }

    #[test]
    fn calc_min_buttons_machine_02() {
        let machine =
            Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        assert_eq!(machine.calc_min_buttons(), 12);
    }

    #[test]
    fn calc_min_buttons_machine_03() {
        let machine =
            Machine::try_from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
                .unwrap();
        assert_eq!(machine.calc_min_buttons(), 11);
    }

    #[test]
    fn calc_min_buttons_machine_04() {
        let machine =
    Machine::try_from("[.#.##.##.#] (9,8,6,4,3,2) (9,7,6,5,4,3,1) (9,7,5,3,1,0) (9,4,3,2) (9,3) (8,7,6,4,2,0) (8,5,3,2,1,0) (8,5,3,2,1) (7,6,5,2,1,0) (7,6,4,1) (7,4,2,1) (7,1) (0) {159,212,79,188,77,173,55,192,53,157}")
    .unwrap();
        assert_eq!(machine.calc_min_buttons(), 240);
    }
}

fn fold(machine_list: &[Machine]) -> u32 {
    machine_list.iter().map(Machine::calc_min_buttons).sum()

    // variables! {
    // vars:
    // n >= 0;
    // p >= 0;
    // q >= 0;
    // r >= 0;
    // s >= 0;
    // t >= 0;
    // } // variables can also be added dynamically with ProblemVariables::add
    // //
    // let solution = vars
    // .minimise(n + p + q + r + s + t)
    // .using(default_solver) // IBM's coin_cbc by default
    // .with(constraint!(n + p + q >= 7))
    // .with(constraint!(n + r + s >= 4))
    // .with(constraint!(p + t >= 5))
    // .with(constraint!(r + t >= 3))
    // // .with(constraint!(1 + a >= 4 - b)) // .with_all(iter) is also available
    // .solve()
    // .expect("Should work");
    // println!(
    // "n={} p={} q={} r={} s={} t={}",
    // solution.value(n),
    // solution.value(p),
    // solution.value(q),
    // solution.value(r),
    // solution.value(s),
    // solution.value(t)
    // );
    // println!("Sum = {}", solution.eval(n + p + q + r + s + t));
    // 0
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
        assert_eq!(fold(&machine_list), 33);
    }

    // #[test]
    // fn fold_final() {
    // let machine_list = parse_content(&get_file_content("assets/input"));
    // assert_eq!(fold(&machine_list), 438);
    // }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let machine_list = parse_content(&file_content);
    println!("Result: {}", fold(&machine_list));
}

// 15659 is too low
// 15663 is too low
