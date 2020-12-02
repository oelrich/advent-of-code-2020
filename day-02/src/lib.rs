use std::fmt::Display;

use tracing::{error, info, warn};
#[derive(Debug, PartialEq)]
pub struct Policy {
    range: core::ops::RangeInclusive<i32>,
    letter: char,
}
impl Display for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{} {}",
            self.range.start(),
            self.range.end(),
            self.letter
        )
    }
}

impl Policy {
    pub fn valid_v0(&self, password: &str) -> bool {
        let count = password.matches(self.letter).count() as i32;
        self.range.contains(&count)
    }
    pub fn valid_v1(&self, password: &str) -> bool {
        let start = (*self.range.start() - 1) as usize;
        let end = (*self.range.end() - 1) as usize;
        let sum = password
            .chars()
            .nth(start)
            .map_or(0, |l| if l == self.letter { 1 } else { 0 })
            + password
                .chars()
                .nth(end)
                .map_or(0, |l| if l == self.letter { 1 } else { 0 });
        sum == 1
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    policy: Policy,
    password: String,
}
impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.policy, self.password)
    }
}

impl Entry {
    pub fn valid_v0(&self) -> bool {
        self.policy.valid_v0(&self.password)
    }
    pub fn valid_v1(&self) -> bool {
        self.policy.valid_v1(&self.password)
    }
}

mod parse {
    use nom::{
        character::complete::{alphanumeric1, anychar, char, digit1, space0},
        combinator::{map_res, recognize},
        sequence::{preceded, separated_pair},
        IResult,
    };

    fn get_range(input: &str) -> IResult<&str, core::ops::RangeInclusive<i32>> {
        use std::str::FromStr;
        let (input, (value0, value1)) = preceded(
            space0,
            separated_pair(
                preceded(space0, map_res(recognize(digit1), i32::from_str)),
                preceded(space0, char('-')),
                preceded(space0, map_res(recognize(digit1), i32::from_str)),
            ),
        )(input)?;
        Ok((input, core::ops::RangeInclusive::new(value0, value1)))
    }

    fn get_policy(input: &str) -> IResult<&str, super::Policy> {
        let (input, (range, letter)) =
            preceded(space0, separated_pair(get_range, space0, anychar))(input)?;
        Ok((input, super::Policy { range, letter }))
    }

    pub fn get_entry(input: &str) -> IResult<&str, super::Entry> {
        let (input, policy) = get_policy(input)?;
        let (input, _colon) = preceded(space0, char(':'))(input)?;
        let (input, password) = preceded(space0, alphanumeric1)(input)?;
        Ok((
            input,
            super::Entry {
                policy,
                password: format!("{}", password),
            },
        ))
    }
}

pub fn count_valid(file_name: &std::path::PathBuf) -> i32 {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let file = File::open(file_name).expect("could not open file");
    let reader = BufReader::new(file);
    // let lines = reader.lines().expect("should have lines ...");
    let mut count = 0;
    for line in reader.lines() {
        match parse::get_entry(&line.expect("a line")) {
            Ok((_rest, entry)) => {
                if entry.valid_v0() {
                    count += 1;
                    info!("{}", entry)
                } else {
                    warn!("{}", entry)
                }
            }
            Err(err) => error!("{}", err),
        }
    }
    count
}

pub fn count_valid_1(file_name: &std::path::PathBuf) -> i32 {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let file = File::open(file_name).expect("could not open file");
    let reader = BufReader::new(file);
    // let lines = reader.lines().expect("should have lines ...");
    let mut count = 0;
    for line in reader.lines() {
        match parse::get_entry(&line.expect("a line")) {
            Ok((_rest, entry)) => {
                if entry.valid_v1() {
                    count += 1;
                    info!("{}", entry)
                } else {
                    warn!("{}", entry)
                }
            }
            Err(err) => error!("{}", err),
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn entry() {
        let (should_be_empty, entry) =
            super::parse::get_entry("1-3 a: abcde").expect("should be valid");
        assert!(should_be_empty.is_empty());
        assert_eq!(
            entry,
            super::Entry {
                policy: super::Policy {
                    range: 1..=3,
                    letter: 'a'
                },
                password: format!("abcde")
            }
        )
    }

    #[test]
    fn small_0() {
        let (should_be_empty, entry) =
            super::parse::get_entry("1-3 a: abcde").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(entry.valid_v0());
    }
    #[test]
    fn small_1() {
        let (should_be_empty, entry) =
            super::parse::get_entry("1-3 b: cdefg").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(!entry.valid_v0());
    }
    #[test]
    fn small_2() {
        let (should_be_empty, entry) =
            super::parse::get_entry("2-9 c: ccccccccc").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(entry.valid_v0());
    }
    #[test]
    fn small_o_0() {
        let (should_be_empty, entry) =
            super::parse::get_entry("1-3 a: abcde").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(entry.valid_v1());
    }
    #[test]
    fn small_o_1() {
        let (should_be_empty, entry) =
            super::parse::get_entry("1-3 b: cdefg").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(!entry.valid_v1());
    }
    #[test]
    fn small_o_2() {
        let (should_be_empty, entry) =
            super::parse::get_entry("2-9 c: ccccccccc").expect("should be good");
        assert!(should_be_empty.is_empty());
        assert!(!entry.valid_v1());
    }
}
