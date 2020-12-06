use std::collections::HashSet;
use tracing::{error, warn};
#[derive(Debug)]
pub struct Form(Vec<char>);
impl Form {
    pub fn as_set(&self) -> HashSet<char> {
        self.0.iter().cloned().collect()
    }
}
#[derive(Debug)]
pub struct Group(Vec<Form>);

impl Group {
    pub fn all_agree_count(&self) -> usize {
        if let Some(remains) = self.0.iter().nth(0).map(|f| f.as_set()) {
            self.0
                .iter()
                .skip(1)
                .fold(remains, |a, b| {
                    a.intersection(&b.as_set()).cloned().collect()
                })
                .len()
        } else {
            0
        }
    }
    pub fn answer_count(&self) -> usize {
        self.0
            .iter()
            .fold(HashSet::default(), |a, b| {
                a.union(&b.as_set()).cloned().collect()
            })
            .len()
    }
}

pub fn groups_from_str(input: &str) -> Vec<Group> {
    match parse::get_groups(input) {
        Ok((rem, groups)) => {
            if !rem.is_empty() {
                warn!("{}", rem);
            }
            groups
        }
        Err(err) => {
            error!("{}", err);
            vec![]
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn intersects() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let groups = super::groups_from_str(input);
        assert_eq!(groups.len(), 5);
        assert_eq!(
            groups
                .iter()
                .map(|g| g.all_agree_count())
                .collect::<Vec<usize>>(),
            vec![3, 0, 1, 1, 1]
        );
    }
}
mod parse {
    use nom::{
        character::{complete::newline, is_alphabetic},
        multi::{many1, separated_list0},
        sequence::tuple,
        IResult,
    };
    fn single_alpha(input: &str) -> IResult<&str, char> {
        let letter = input.bytes().nth(0);

        if let Some(letter) = letter {
            if is_alphabetic(letter) {
                return Ok((&input[1..], letter as char));
            }
        }
        Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::NoneOf,
        )))
    }
    fn get_form_answers(input: &str) -> IResult<&str, super::Form> {
        let (input, answers) = many1(single_alpha)(input)?;
        Ok((input, super::Form(answers)))
    }
    fn get_group_answers(input: &str) -> IResult<&str, super::Group> {
        let (input, forms) = separated_list0(newline, get_form_answers)(input)?;
        Ok((input, super::Group(forms)))
    }
    pub fn get_groups(input: &str) -> IResult<&str, Vec<super::Group>> {
        separated_list0(tuple((newline, newline)), get_group_answers)(input)
    }
    #[cfg(test)]
    mod tests {
        #[test]
        fn groups_0() {
            let (rem, groups) = super::get_groups("abc").expect("group");
            assert!(rem.is_empty());
            assert_eq!(groups.len(), 1);
            assert_eq!(groups.iter().nth(0).unwrap().answer_count(), 3);
        }
        #[test]
        fn groups_1() {
            let (rem, groups) = super::get_groups("abc\n\nab").expect("group");
            assert!(rem.is_empty());
            assert_eq!(groups.len(), 2);
            assert_eq!(groups.iter().nth(0).unwrap().answer_count(), 3);
            assert_eq!(groups.iter().nth(1).unwrap().answer_count(), 2);
        }
        #[test]
        fn groups_2() {
            let (rem, groups) = super::get_groups("ab\nc\n\na\nbc").expect("group");
            assert!(rem.is_empty());
            assert_eq!(groups.len(), 2);
            assert_eq!(groups.iter().nth(0).unwrap().answer_count(), 3);
            assert_eq!(groups.iter().nth(1).unwrap().answer_count(), 3);
        }
        #[test]
        fn group_0() {
            let (rem, group) = super::get_group_answers("abc").expect("group");
            assert!(rem.is_empty());
            assert_eq!(group.0.len(), 1);
            assert_eq!(group.answer_count(), 3);
        }
        #[test]
        fn group_1() {
            let (rem, group) = super::get_group_answers("abc\nab").expect("group");
            assert!(rem.is_empty());
            assert_eq!(group.0.len(), 2);
            assert_eq!(group.answer_count(), 3);
        }
        #[test]
        fn group_2() {
            let (rem, group) = super::get_group_answers("abc\ndef\ngnu").expect("group");
            assert!(rem.is_empty());
            assert_eq!(group.0.len(), 3);
            assert_eq!(group.answer_count(), 9);
        }
    }
}
