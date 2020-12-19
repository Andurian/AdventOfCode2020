#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

trait Rule {
    /// Return all strings that can be produced by that rule.
    /// May not terminate if rules contain recursions.
    fn valid_strings(&self, set: &RuleSet) -> Vec<String>;

    /// Tries to consume the part of the string that adheres to the rule.
    /// If multiple ways to consume the string are possible, all ways are returned.
    /// If the return value contains the empty string, the entire string could be consumed.
    /// This works with recursive rules as long as there is always at least one character
    /// consumed before the recursive rule is invocated.
    fn try_consume(&self, s: &str, set: &RuleSet) -> Vec<String>;
}

struct LiteralRule {
    literal: String,
}

impl Rule for LiteralRule {
    fn valid_strings(&self, _set: &RuleSet) -> Vec<String> {
        vec![self.literal.clone()]
    }

    fn try_consume(&self, s: &str, _set: &RuleSet) -> Vec<String> {
        if self.literal.len() > s.len() {
            return Vec::new();
        }
        if s[..self.literal.len()] == self.literal {
            return vec![String::from(&s[self.literal.len()..])];
        }
        return Vec::new();
    }
}

struct ComplexRule {
    subrules: Vec<Vec<i32>>,
}

impl Rule for ComplexRule {
    fn valid_strings(&self, set: &RuleSet) -> Vec<String> {
        let mut ret = Vec::new();

        for subrule in &self.subrules {
            let mut intermediate = vec![String::new()];
            for rule in subrule {
                intermediate = iproduct!(intermediate.iter(), set.valid_strings(*rule))
                    .map(|(s1, s2)| {
                        let mut s = String::from(s1);
                        s.push_str(&s2);
                        s
                    })
                    .collect::<Vec<String>>();
            }
            ret.append(&mut intermediate);
        }

        ret
    }

    fn try_consume(&self, s: &str, set: &RuleSet) -> Vec<String> {
        let mut ret = Vec::new();

        if s == "" {
            return ret;
        }

        for subrule in &self.subrules {
            let mut to_match = vec![String::from(s)];
            for rule in subrule {
                let mut matching_results = Vec::new();
                for candidate in to_match {
                    matching_results
                        .append(&mut set.rules.get(rule).unwrap().try_consume(&candidate, set));
                }
                to_match = matching_results;
            }
            ret.append(&mut to_match);
        }

        ret
    }
}

struct RuleSet {
    rules: HashMap<i32, Box<dyn Rule>>,
}

impl RuleSet {
    fn from_vec(lines: Vec<&str>) -> RuleSet {
        let mut rules = HashMap::new();

        'outer: for line in lines {
            let mut token_iter = line.split(':');
            let rule_id = token_iter.next().unwrap().parse::<i32>().unwrap();
            let rules_iter = token_iter.next().unwrap().split('|').map(|s| s.trim());
            let mut subrule_ids = Vec::<Vec<i32>>::new();
            for subrule in rules_iter {
                if subrule.chars().next().unwrap() == '"' {
                    rules.insert(
                        rule_id,
                        Box::new(LiteralRule {
                            literal: String::from(subrule.split("\"").skip(1).next().unwrap()),
                        }) as Box<dyn Rule>,
                    );
                    continue 'outer;
                }
                subrule_ids.push(
                    subrule
                        .split(" ")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }

            if !subrule_ids.is_empty() {
                rules.insert(
                    rule_id,
                    Box::new(ComplexRule {
                        subrules: subrule_ids,
                    }) as Box<dyn Rule>,
                );
            }
        }

        RuleSet { rules }
    }

    fn valid_strings(&self, rule_id: i32) -> Vec<String> {
        self.rules.get(&rule_id).unwrap().valid_strings(&self)
    }

    fn does_match(&self, s: &str, rule_id: i32) -> bool {
        let ret = self.rules.get(&rule_id).unwrap().try_consume(s, self);
        ret.iter().any(|x| x == "")
    }
}

fn count_valid_words_exhaustive(filename: &str, rule_to_match: i32) -> i32 {
    let groups = common::read_grouped_file(filename);
    let rules = RuleSet::from_vec(groups[0].split("\n").collect::<Vec<&str>>());
    let valid_strings = rules.valid_strings(rule_to_match);
    let candidate_strings = groups[1]
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let valid_strings = HashSet::<String>::from_iter(valid_strings.iter().cloned());
    let candidate_strings = HashSet::<String>::from_iter(candidate_strings.iter().cloned());

    valid_strings
        .intersection(&candidate_strings)
        .cloned()
        .collect::<HashSet<String>>()
        .len() as i32
}

fn count_valid_words(filename: &str, rule_to_match: i32) -> i32 {
    let groups = common::read_grouped_file(filename);
    let rules = RuleSet::from_vec(groups[0].split("\n").collect::<Vec<&str>>());
    groups[1].split("\n").fold(0, |acc, s| {
        if rules.does_match(s, rule_to_match) {
            return acc + 1;
        }
        acc
    })
}

fn main() {
    println!(
        "Solution 1 (exhaustive): {}",
        count_valid_words_exhaustive("src/day19/input.txt", 0)
    );
    println!(
        "Solution 1: {}",
        count_valid_words("src/day19/input.txt", 0)
    );
    println!(
        "Solution 2: {}",
        count_valid_words("src/day19/input_recursive.txt", 0)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_valid_words_exhaustive() {
        assert_eq!(
            count_valid_words_exhaustive("src/day19/input_test.txt", 0),
            2
        );
    }

    #[test]
    fn test_count_valid_words() {
        assert_eq!(count_valid_words("src/day19/input_test.txt", 0), 2);
    }

    #[test]
    fn test_count_valid_words_recursive() {
        assert_eq!(
            count_valid_words("src/day19/input_test_recursive.txt", 0),
            12
        );
    }
}
