use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = include_str!("test.txt");
const FULL_INPUT: &str = include_str!("full.txt");

peg::parser! {
    grammar rule_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("usize"))}

        /// Parses a rule indicating that one page comes before another.
        pub rule comes_before() -> OrderedBeforeRule
            = page_number:number() "|" comes_before:number() {
                OrderedBeforeRule {
                    page_number,
                    comes_before
                }
        }

        /// Parses multiple "comes before" page rules
        pub rule page_rules() -> HashMap<usize, Vec<OrderedBeforeRule>>
            = rules:(comes_before() ** "\n") {
            rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<usize, Vec<OrderedBeforeRule>>, item| {
                if let Some(items) = acc.get_mut(&item.page_number) {
                    items.push(item);
                } else {
                    acc.insert(item.page_number, vec![item]);
                }
                acc
            })
        }

        pub rule update() -> PageUpdate
            = pages:(number() ** ",") { PageUpdate(pages) }

        pub rule update_seq() -> Vec<PageUpdate>
            = updates:(update() ** "\n") { updates }

        pub rule problem_input() -> (HashMap<usize, Vec<OrderedBeforeRule>>, Vec<PageUpdate>)
            = rules:page_rules() "\n"+ updates:update_seq() { (rules, updates) }
    }
}

fn part1(input: &str) -> usize {
    let (rules, updates) = rule_parser::problem_input(input).expect("Unable to parse input file");

    updates
        .iter()
        .filter_map(|update| {
            if update.is_valid(&rules) {
                Some(update.0[update.0.len() / 2])
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let (rules, updates) = rule_parser::problem_input(input).expect("Unable to parse input file");

    updates
        .into_iter()
        .filter_map(|update| {
            if update.is_valid(&rules) {
                None
            } else {
                let valid = update.to_valid(&rules);
                Some(valid.0[valid.0.len() / 2])
            }
        })
        .sum::<usize>()
}

fn main() {
    println!(
        "P1 - test: {} full: {}",
        part1(TEST_INPUT),
        part1(FULL_INPUT)
    );

    println!(
        "P2 - test: {} full: {}",
        part2(TEST_INPUT),
        part2(FULL_INPUT)
    );
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct OrderedBeforeRule {
    page_number: usize,
    comes_before: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PageUpdate(Vec<usize>);

impl PageUpdate {
    fn is_valid(&self, rules: &HashMap<usize, Vec<OrderedBeforeRule>>) -> bool {
        self.0
            .iter()
            .fold((HashSet::new(), true), |(mut seen, valid), page| {
                if valid {
                    let obeys_rules = rules.get(page).map_or(true, |page_rules| {
                        page_rules
                            .iter()
                            .all(|rule| !seen.contains(&rule.comes_before))
                    });
                    seen.insert(*page);
                    (seen, obeys_rules)
                } else {
                    // Short circuit
                    (seen, valid)
                }
            })
            .1 // .1 = tuple index 1
    }

    fn to_valid(&self, rules: &HashMap<usize, Vec<OrderedBeforeRule>>) -> Self {
        let mut valid = self.clone();

        // Surely there's a better way than iteratively re-solving,
        // but for AoC I'm too lazy to "properly" do this
        // ensuring a minimal number of updates
        while !valid.is_valid(rules) {
            let (page_indices, violations) = valid.0.iter().enumerate().fold(
                (HashMap::new(), Vec::new()),
                |(mut seen, mut violations), (i, page)| {
                    let violated_rules = rules.get(page).map_or(Vec::new(), |page_rules| {
                        page_rules
                            .iter()
                            .filter(|rule| seen.contains_key(&rule.comes_before))
                            .cloned()
                            .collect()
                    });

                    if !seen.contains_key(page) {
                        seen.insert(*page, i);
                    }

                    if !violated_rules.is_empty() {
                        violations.push((violated_rules, i));
                    }

                    (seen, violations)
                },
            );

            for (rules, violation_index) in violations.iter() {
                let new_index = rules
                    .iter()
                    .flat_map(|v| page_indices.get(&v.comes_before))
                    .min()
                    .expect("Programmer error");

                let value = valid.0.remove(*violation_index);
                valid.0.insert(*new_index, value);
            }
        }

        valid
    }
}

#[cfg(test)]
mod test {
    use super::{rule_parser, OrderedBeforeRule, PageUpdate, FULL_INPUT, TEST_INPUT};
    use std::collections::HashMap;

    #[test]
    fn test_comes_before_parser() {
        assert_eq!(
            rule_parser::comes_before("47|53"),
            Ok(OrderedBeforeRule {
                page_number: 47,
                comes_before: 53,
            })
        );
    }

    #[test]
    fn test_multiple_rules_parser() {
        assert_eq!(
            rule_parser::page_rules("47|53\n97|13\n97|61"),
            Ok(HashMap::from([
                (
                    47,
                    vec![OrderedBeforeRule {
                        page_number: 47,
                        comes_before: 53,
                    }]
                ),
                (
                    97,
                    vec![
                        OrderedBeforeRule {
                            page_number: 97,
                            comes_before: 13,
                        },
                        OrderedBeforeRule {
                            page_number: 97,
                            comes_before: 61,
                        },
                    ]
                )
            ]))
        );
    }

    #[test]
    fn test_update_parser() {
        assert_eq!(
            rule_parser::update("75,47,61,53,29"),
            Ok(PageUpdate(vec![75, 47, 61, 53, 29]))
        )
    }

    #[test]
    fn test_update_seq_parser() {
        assert_eq!(
            rule_parser::update_seq("75,47,61,53,29\n97,61,53,29,13"),
            Ok(vec![
                PageUpdate(vec![75, 47, 61, 53, 29]),
                PageUpdate(vec![97, 61, 53, 29, 13]),
            ])
        )
    }

    #[test]
    fn test_input_parser() {
        // Can we parse the problem input?
        assert!(rule_parser::problem_input(TEST_INPUT).is_ok());
        assert!(rule_parser::problem_input(FULL_INPUT).is_ok());
    }
}
