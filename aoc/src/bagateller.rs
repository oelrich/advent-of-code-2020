use std::collections::{HashMap, HashSet};
fn shiny_count_rec(bag: &str, truth: &HashMap<String, HashMap<String, usize>>) -> usize {
    truth
        .get(bag)
        .map(|hm| {
            println!("{}", bag);
            let result = 0;
            if let Some(entries) = truth.get(bag) {
                //  entries.iter().map(|(e, u)| 2)
            }
            hm.iter()
                .fold(0, |v, (b, c)| v + *c * shiny_count_rec(b, truth))
        })
        .unwrap_or(1)
}
pub fn shiny_count(input: &str) -> usize {
    match parse::get_bags(input) {
        Ok((remainder, entries)) => {
            if remainder.is_empty() {
                let bags: HashMap<String, HashMap<String, usize>> =
                    entries.iter().cloned().collect();

                let bag = format!("shiny gold");

                shiny_count_rec(&bag, &bags)
            } else {
                error!("{}", remainder);
                0
            }
        }
        Err(err) => {
            error!("{}", err);
            0
        }
    }
}

fn contains_shiny_gold_rec(
    bag: &str,
    known: &HashSet<String>,
    truth: &HashMap<String, HashMap<String, usize>>,
) -> bool {
    known.contains(bag)
        || truth
            .get(bag)
            .cloned()
            .map(|hm| {
                hm.iter()
                    .any(|(b, _)| contains_shiny_gold_rec(b, known, truth))
            })
            .unwrap_or(false)
}

use tracing::error;
pub fn shiny_gold(input: &str) -> usize {
    match parse::get_bags(input) {
        Ok((remainder, entries)) => {
            if remainder.is_empty() {
                let bags: HashMap<String, HashMap<String, usize>> =
                    entries.iter().cloned().collect();
                let mut contains_shiny_gold: HashSet<String> = HashSet::default();
                contains_shiny_gold.insert(format!("shiny gold"));
                for bag in entries.iter().map(|(bag, _content)| bag.clone()) {
                    if contains_shiny_gold_rec(&bag, &contains_shiny_gold, &bags) {
                        contains_shiny_gold.insert(bag);
                    }
                }

                contains_shiny_gold.len() - 1
            } else {
                error!("{}", remainder);
                0
            }
        }
        Err(err) => {
            error!("{}", err);
            0
        }
    }
}
#[cfg(test)]
mod tests {

    const RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    #[test]
    fn count() {
        assert_eq!(super::shiny_gold(RULES), 4)
    }

    const SHINY: &str = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
    #[test]
    fn shiny_count() {
        assert_eq!(super::shiny_count(RULES), 32)
    }
    #[test]
    fn shiny() {
        assert_eq!(super::shiny_count(SHINY), 126)
    }
}
mod parse {
    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, newline},
        combinator::map_res,
        multi::{separated_list0, separated_list1},
        sequence::separated_pair,
        IResult,
    };
    fn get_description(input: &str) -> IResult<&str, String> {
        let (input, (qualifier, colour)) = separated_pair(alpha1, char(' '), alpha1)(input)?;
        Ok((input, format!("{} {}", qualifier, colour)))
    }
    fn get_count_description(input: &str) -> IResult<&str, (String, usize)> {
        use std::str::FromStr;
        let (input, (count, desc)) =
            separated_pair(map_res(digit1, usize::from_str), char(' '), get_description)(input)?;
        let (input, _) = alt((tag(" bags"), tag(" bag")))(input)?;
        Ok((input, (desc, count)))
    }
    fn get_no_bags(input: &str) -> IResult<&str, Vec<(String, usize)>> {
        let (input, _no_other_bags) = tag("no other bags")(input)?;
        Ok((input, vec![]))
    }
    fn get_content(input: &str) -> IResult<&str, Vec<(String, usize)>> {
        let (input, count_description) = alt((
            get_no_bags,
            separated_list1(alt((tag(" "), tag(", "))), get_count_description),
        ))(input)?;
        Ok((input, count_description))
    }

    fn get_bag(input: &str) -> IResult<&str, (String, HashMap<String, usize>)> {
        let (input, (description, capacity)) =
            separated_pair(get_description, tag(" bags contain "), get_content)(input)?;
        let (input, _done) = tag(".")(input)?;
        Ok((input, (description, capacity.iter().cloned().collect())))
    }
    pub fn get_bags(input: &str) -> IResult<&str, Vec<(String, HashMap<String, usize>)>> {
        separated_list0(newline, get_bag)(input)
    }
    #[cfg(test)]
    mod test {

        const RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        #[test]
        fn get_all() {
            let (input, bags) = super::get_bags(RULES).expect("parse");
            assert_eq!(input, "");
            assert_eq!(bags.len(), 9);
            let bag_count = vec![3, 7, 1, 11, 3, 7, 11, 0, 0];
            let actual: Vec<usize> = bags
                .iter()
                .map(|b| b.1.iter().fold(0, |a, (_, c)| a + c))
                .collect();
            assert_eq!(bag_count, actual);
        }
    }
}
