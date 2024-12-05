use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type OrderingRule = (usize, usize);
type OrderingRuleList = Vec<OrderingRule>;
type Update = Vec<usize>;
type UpdateList = Vec<Update>;

#[derive(Debug, PartialEq, Clone)]
struct Content {
    ordering_rule_list: OrderingRuleList,
    update_list: UpdateList,
}

fn parse_content(input: &str) -> Content {
    // Input is comprised of two big blocks:
    // - The list of ordering rules
    // - The list of pages to produce
    let mut block = input.split("\n\n");

    let ordering_rule_list: OrderingRuleList = block
        .next()
        .expect("There should be an ordering rules block")
        .split_whitespace()
        .map(|ordering_rule_line| {
            let mut line_split = ordering_rule_line.split("|");
            (
                line_split
                    .next()
                    .expect("Cannot read ordering rule first argument")
                    .parse()
                    .expect("Cannot parse ordering rule first argument"),
                line_split
                    .next()
                    .expect("Cannot read ordering rule second argument")
                    .parse()
                    .expect("Cannot parse ordering rule second argument"),
            )
        })
        .collect();

    let update_list: UpdateList = block
        .next()
        .expect("There should be an pages to produce block")
        .split_whitespace()
        .map(|line| {
            line.split(",")
                .map(|e| e.parse().expect("Cannot parse page to produce"))
                .collect::<Vec<usize>>()
        })
        .collect();

    Content {
        ordering_rule_list,
        update_list,
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
47|53
97|13
97|61

75,47,61
97,61
",
        );
        assert_eq!(
            content,
            Content {
                ordering_rule_list: vec![(47, 53), (97, 13), (97, 61)],
                update_list: vec![vec![75, 47, 61], vec![97, 61]]
            }
        );
    }
}

fn is_correctly_ordered(ordering_rule_list: &OrderingRuleList, update: &Update) -> Option<Update> {
    if ordering_rule_list
        .iter()
        .map(|rule| {
            // Does the update violates the current rule ?
            // Let's find out !
            let first = update.iter().position(|&e| e == rule.0);
            let second = update.iter().position(|&e| e == rule.1);
            match (first, second) {
                (Some(f), Some(s)) => f < s,
                (_, _) => true,
            }
        })
        .reduce(|acc, cur| acc && cur)
        .expect("There should be ordering rules to parse")
    {
        Some(update.to_owned())
    } else {
        None
    }
}

#[cfg(test)]
mod tests_is_correctly_ordered {
    use super::*;

    #[test]
    fn is_correctly_ordered_01() {
        let content = parse_content(
            "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![75, 47, 61, 53, 29]),
            Some(vec![75, 47, 61, 53, 29])
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![97, 61, 53, 29, 13]),
            Some(vec![97, 61, 53, 29, 13])
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![75, 29, 13]),
            Some(vec![75, 29, 13])
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![75, 97, 47, 61, 53]),
            None
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![61, 13, 29]),
            None
        );
        assert_eq!(
            is_correctly_ordered(&content.ordering_rule_list, &vec![97, 13, 75, 29, 47]),
            None
        );
    }
}

fn fold(content: &Content) -> usize {
    content
        .update_list
        .iter()
        .filter_map(|update| is_correctly_ordered(&content.ordering_rule_list, update))
        .map(|update| update[update.len() / 2])
        .reduce(|acc, cur| acc + cur)
        .expect("Update list should not be empty")
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );
        assert_eq!(fold(&content), 143);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(content));
}
