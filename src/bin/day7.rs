use aoc2020::read_string_input;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let contents = read_string_input("in-data/day7.txt");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &str) -> usize {
    let input = parse_input_part_1(&contents);

    let mut items_to_check = vec!["shiny gold"];
    let mut bags_that_can_hold_shiny_gold_bags = HashSet::new();

    loop {
        if items_to_check.is_empty() {
            break;
        }
        let item = items_to_check.remove(0);
        if let Some(new_items) = input.get(item) {
            for n in new_items {
                items_to_check.push(n);
                bags_that_can_hold_shiny_gold_bags.insert(n);
            }
        }
    }
    bags_that_can_hold_shiny_gold_bags.len()
}

fn parse_input_part_1(contents: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let re = Regex::new(r"^(?P<key>\w+ \w+) bags contain (?P<val>[^.]+)").unwrap();

    for rule in contents.lines() {
        let captures = re.captures(rule).unwrap();
        let vals = captures.name("val").unwrap().as_str();
        for rule in find_content_colors(vals) {
            let s = String::from(captures.name("key").unwrap().as_str());
            match map.get_mut(&rule.color) {
                Some(v) => {
                    v.push(s);
                }
                None => {
                    map.insert(rule.color, vec![s]);
                }
            }
        }
    }

    map
}

fn find_content_colors(content: &str) -> Vec<BagContents> {
    if content == "no other bags" {
        return vec![];
    }
    content
        .split(", ")
        .map(|s| s.parse::<BagContents>().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct BagContents {
    amount: usize,
    color: String,
}

impl FromStr for BagContents {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<amount>\d+) (?P<color>\w+ \w+)").unwrap();
        let captures = re.captures(s).unwrap();
        Ok(BagContents {
            amount: captures.name("amount").unwrap().as_str().parse().unwrap(),
            color: String::from(captures.name("color").unwrap().as_str()),
        })
    }
}

#[derive(Debug, Clone)]
struct BagRule {
    color: String,
    rules: Vec<BagContents>,
}

impl FromStr for BagRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<contents>[^.]+)").unwrap();
        let captures = re.captures(s).unwrap();

        let color = String::from(captures.name("color").unwrap().as_str());
        let rules = match captures.name("contents") {
            Some(contents) => {
                if contents.as_str() == "no other bags" {
                    vec![]
                } else {
                    contents
                        .as_str()
                        .split(", ")
                        .map(|s| s.parse::<BagContents>().unwrap())
                        .collect()
                }
            }
            None => vec![],
        };

        Ok(BagRule { color, rules })
    }
}

fn part_2(contents: &str) -> usize {
    let rules = parse_input_part_2(&contents);

    count_bags(&rules, "shiny gold") - 1
}

fn parse_input_part_2(input: &str) -> HashMap<String, BagRule> {
    let mut map = HashMap::new();
    for rule in input.split('\n').map(|l| l.parse::<BagRule>().unwrap()) {
        map.insert(String::from(&rule.color), rule);
    }
    map
}

fn count_bags(rules: &HashMap<String, BagRule>, color: &str) -> usize {
    1 + match rules.get(color) {
        Some(rule) => {
            if rule.rules.is_empty() {
                0
            } else {
                rule.rules
                    .iter()
                    .map(|r| r.amount * count_bags(rules, &r.color))
                    .sum::<usize>()
            }
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_find_content_colors() {
    //     let no_other_bags_exepcted: Vec<String> = vec![];
    //     assert_eq!(
    //         find_content_colors(&"no other bags"),
    //         no_other_bags_exepcted
    //     );

    //     assert_eq!(
    //         find_content_colors(&"1 light brown bag"),
    //         vec!["light brown"]
    //     );

    //     assert_eq!(
    //         find_content_colors(&"1 light brown bag, 2 bright red bags"),
    //         vec!["light brown", "bright red"]
    //     )
    // }

    #[test]
    fn test_parse_input_part_1() {
        let parsed = parse_input_part_1(&"bright red bags contain 2 light cyan bags");

        assert!(parsed.contains_key("light cyan"));
    }

    #[test]
    fn test_sample_input_part_1() {
        let sample_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(part_1(&sample_input), 4);
    }

    #[test]
    fn test_count_bags() {
        let sample_input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let rules = parse_input_part_2(&sample_input);

        assert_eq!(count_bags(&rules, "dark violet"), 1);
        assert_eq!(count_bags(&rules, "dark blue"), 3);
        assert_eq!(count_bags(&rules, "dark green"), 7);
        assert_eq!(count_bags(&rules, "dark yellow"), 15);
        assert_eq!(count_bags(&rules, "dark orange"), 31);
        assert_eq!(count_bags(&rules, "dark red"), 63);
        assert_eq!(count_bags(&rules, "shiny gold"), 127);
    }

    #[test]
    fn test_sample_input_part_2() {
        let sample_input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(part_2(&sample_input), 126);
    }
}
