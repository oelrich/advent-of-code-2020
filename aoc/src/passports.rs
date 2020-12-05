use tracing::{error, warn};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BirthYear(i32);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IssueYear(i32);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExpirationYear(i32);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Height {
    Cm(i32),
    In(i32),
    Un(i32),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Colour {
    Text(String),
    RGB(u8, u8, u8),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EyeColour(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    Text(String),
    Numeric(isize),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PassportId(Id);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CountryId(Id);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PassportEntry {
    BirthYear(BirthYear),
    IssueYear(IssueYear),
    ExpirationYear(ExpirationYear),
    Height(Height),
    HairColour(Colour),
    EyeColour(Colour),
    PassportId(PassportId),
    CountryId(CountryId),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PassportKey {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColour,
    EyeColour,
    PassportId,
    CountryId,
}
#[derive(Debug)]
pub struct PassportError(&'static str);

#[derive(Debug, Clone)]
pub struct Passport(std::collections::HashMap<PassportKey, PassportEntry>);
impl Passport {
    pub fn passport_from_entries(entries: Vec<(PassportKey, PassportEntry)>) -> Option<Self> {
        let size = entries.len();
        if size == 8 {
            return Some(Passport(entries.iter().cloned().collect()));
        }
        if size == 7 {
            let cid = entries.iter().any(|(k, _v)| *k == PassportKey::CountryId);
            if !cid {
                return Some(Passport(entries.iter().cloned().collect()));
            }
        }
        None
    }
    pub fn passports_from_str(input: &str) -> Vec<Self> {
        match parse::get_passports(input) {
            Ok((remainder, passports)) => {
                if !remainder.is_empty() {
                    warn!("Remainder: '{}'", remainder);
                }
                passports
            }
            Err(err) => {
                error!("{}", err);
                vec![]
            }
        }
    }
}
mod parse {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while_m_n},
        character::complete::{alpha1, digit1, newline, one_of, space1},
        combinator::{map_res, opt, recognize},
        multi::{many1, separated_list0, separated_list1},
        sequence::tuple,
        IResult,
    };
    use std::str::FromStr;

    fn get_garbage_text(input: &str) -> IResult<&str, &str> {
        recognize(many1(alt((digit1, alpha1, recognize(one_of("#"))))))(input)
    }

    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
    }

    fn get_hex_colour(input: &str) -> IResult<&str, Colour> {
        let (input, (_, red, green, blue)) =
            tuple((tag("#"), hex_primary, hex_primary, hex_primary))(input)?;
        Ok((input, Colour::RGB(red, green, blue)))
    }

    fn get_hair_colour(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, _) = tag("hcl:")(input)?;
        let (input, text) = get_garbage_text(input)?;
        let (colour_input, hair_colour) = opt(get_hex_colour)(text)?;

        Ok((
            input,
            (
                PassportKey::HairColour,
                hair_colour
                    .filter(|_| colour_input.is_empty())
                    .map(|c| PassportEntry::HairColour(c)),
            ),
        ))
    }

    fn get_isize(input: &str) -> IResult<&str, isize> {
        map_res(recognize(digit1), isize::from_str)(input)
    }
    fn get_pid(input: &str) -> IResult<&str, Option<Id>> {
        let (input, text) = get_garbage_text(input)?;
        if text.len() == 9 {
            let (is_input, is_id) = opt(get_isize)(text)?;
            if is_input.is_empty() {
                if let Some(id) = is_id {
                    return Ok((input, Some(Id::Numeric(id))));
                }
            }
        }
        Ok((input, None))
    }

    fn get_passport_id(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, pid)) = tuple((tag("pid:"), get_pid))(input)?;
        Ok((
            input,
            (
                PassportKey::PassportId,
                pid.map(|p| PassportEntry::PassportId(PassportId(p))),
            ),
        ))
    }
    fn get_quad_digit(input: &str) -> IResult<&str, Option<i32>> {
        let (input, text) = get_garbage_text(input)?;
        if text.len() == 4 {
            match str::parse::<i32>(text) {
                Ok(value) => return Ok((input, Some(value))),
                _ => (),
            }
        }
        Ok((input, None))
    }
    fn get_birth_year(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, year)) = tuple((tag("byr:"), get_quad_digit))(input)?;
        Ok((
            input,
            (
                PassportKey::BirthYear,
                year.filter(|v| *v >= 1920)
                    .filter(|v| *v <= 2002)
                    .map(|v| PassportEntry::BirthYear(BirthYear(v))),
            ),
        ))
    }

    fn get_issue_year(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, year)) = tuple((tag("iyr:"), get_quad_digit))(input)?;
        Ok((
            input,
            (
                PassportKey::IssueYear,
                year.filter(|v| *v >= 2010)
                    .filter(|v| *v <= 2020)
                    .map(|v| PassportEntry::IssueYear(IssueYear(v))),
            ),
        ))
    }
    fn get_expiration_year(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, year)) = tuple((tag("eyr:"), get_quad_digit))(input)?;
        Ok((
            input,
            (
                PassportKey::ExpirationYear,
                year.filter(|v| *v >= 2020)
                    .filter(|v| *v <= 2030)
                    .map(|v| PassportEntry::ExpirationYear(ExpirationYear(v))),
            ),
        ))
    }

    fn get_height(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input_hgt, _) = tag("hgt:")(input)?;
        let (input_done, text) = get_garbage_text(input_hgt)?;
        let (_text_rem, height) = opt(tuple((
            map_res(recognize(digit1), i32::from_str),
            alt((tag("cm"), tag("in"))),
        )))(text)?;
        let result;
        if let Some((height, unit)) = height {
            match unit {
                "cm" => {
                    result = if height >= 150 && height <= 193 {
                        Some(PassportEntry::Height(Height::Cm(height)))
                    } else {
                        None
                    }
                }
                "in" => {
                    result = if height >= 59 && height <= 76 {
                        Some(PassportEntry::Height(Height::In(height)))
                    } else {
                        None
                    }
                }
                _ => panic!("unit matched but missing?"),
            };
        } else {
            result = None;
        }
        Ok((input_done, (PassportKey::Height, result)))
    }

    fn get_eye_colour(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, text)) = tuple((tag("ecl:"), get_garbage_text))(input)?;
        let (done_input, colour) = opt(alt((
            tag("amb"),
            tag("gry"),
            tag("grn"),
            tag("hzl"),
            tag("oth"),
            tag("brn"),
            tag("blu"),
        )))(text)?;

        Ok((
            input,
            (
                PassportKey::EyeColour,
                colour
                    .filter(|_| done_input.is_empty())
                    .map(|c| PassportEntry::EyeColour(Colour::Text(format!("{}", c)))),
            ),
        ))
    }
    fn get_cid(input: &str) -> IResult<&str, Option<Id>> {
        let (input, text) = get_garbage_text(input)?;
        let (is_input, is_id) = opt(get_isize)(text)?;
        if is_input.is_empty() {
            if let Some(id) = is_id {
                return Ok((input, Some(Id::Numeric(id))));
            }
        }

        Ok((input, None))
    }
    fn get_country_id(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        let (input, (_, text)) = tuple((tag("cid:"), get_garbage_text))(input)?;
        let (cid_input, pre_cid) = get_cid(text)?;

        let cid = pre_cid
            .clone()
            .filter(|_| cid_input.is_empty())
            .map(|cid| PassportEntry::CountryId(CountryId(cid)));
        match (pre_cid, cid.clone()) {
            (Some(id), None) => warn!("bad cid: {:?}", id),
            _ => (),
        }
        Ok((input, (PassportKey::CountryId, cid)))
    }
    fn get_passport_entry(input: &str) -> IResult<&str, (PassportKey, Option<PassportEntry>)> {
        alt((
            get_hair_colour,
            get_passport_id,
            get_birth_year,
            get_expiration_year,
            get_issue_year,
            get_height,
            get_eye_colour,
            get_country_id,
        ))(input)
    }
    fn get_field_separator(input: &str) -> IResult<&str, &str> {
        alt((recognize(newline), space1))(input)
    }

    fn get_passport_entries(
        input: &str,
    ) -> IResult<&str, Vec<(super::PassportKey, Option<PassportEntry>)>> {
        separated_list1(get_field_separator, get_passport_entry)(input)
    }

    fn get_passport(input: &str) -> IResult<&str, Option<super::Passport>> {
        let (result_input, entries) = get_passport_entries(input)?;
        let entries: Vec<(PassportKey, PassportEntry)> = entries
            .iter()
            .cloned()
            .filter_map(|(p, k)| k.map(|e| (p, e)))
            .collect();
        let passport = Passport::passport_from_entries(entries);
        Ok((result_input, passport))
    }

    fn get_passport_separator(input: &str) -> IResult<&str, &str> {
        recognize(tuple((newline, newline)))(input)
    }

    pub fn get_passports(input: &str) -> IResult<&str, Vec<super::Passport>> {
        let (rem, passports) = separated_list0(get_passport_separator, get_passport)(input)?;
        Ok((rem, passports.iter().cloned().filter_map(|p| p).collect()))
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn passport_split() {
            let (reminder, list) = super::get_passport_separator("\n\n \n\n").expect("parse");
            assert_eq!(reminder, " \n\n");
            assert_eq!(list, "\n\n");
        }
        #[test]
        fn prop_ecl() {
            let (remainder, (pk, pv)) =
                super::get_passport_entry("ecl:gry").expect("parse should work");
            assert_eq!(remainder, "");
            assert_eq!(pk, super::PassportKey::EyeColour);
            assert_eq!(
                pv,
                Some(super::PassportEntry::EyeColour(super::Colour::Text(
                    format!("gry")
                )))
            );
        }
        #[test]
        fn prop_pid() {
            let (remainder, (pk, pv)) =
                super::get_passport_entry("pid:860033327").expect("parse should work");
            assert_eq!(remainder, "");
            assert_eq!(pk, super::PassportKey::PassportId);
            assert_eq!(
                pv,
                Some(super::PassportEntry::PassportId(super::PassportId(
                    super::Id::Numeric(860033327)
                )))
            );
        }
        #[test]
        fn prop_hgt() {
            let (remainder, (pk, pv)) =
                super::get_passport_entry("hgt:183cm").expect("parse should work");
            assert_eq!(remainder, "");
            assert_eq!(pk, super::PassportKey::Height);
            assert_eq!(
                pv,
                Some(super::PassportEntry::Height(super::Height::Cm(183)))
            );
        }
        #[test]
        fn passport_complete() {
            let (reminder, passport) = super::get_passports(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            )
            .expect("parse should work");
            assert_eq!(reminder, "");
            assert_eq!(passport.len(), 1);
        }

        #[test]
        fn passport_incomplete() {
            let (reminder, passport) = super::get_passports(
                "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931 hgt:179cm",
            )
            .expect("parse should work");
            assert_eq!(reminder, "");
            assert_eq!(passport.len(), 1);
        }

        #[test]
        fn passport_entries() {
            use super::*;
            let (reminder, passport) = super::get_passport_entries(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            )
            .expect("parse should work");
            assert_eq!(reminder, "");
            assert_eq!(
                passport,
                vec![
                    (
                        PassportKey::EyeColour,
                        Some(PassportEntry::EyeColour(Colour::Text(format!("gry"))))
                    ),
                    (
                        PassportKey::PassportId,
                        Some(PassportEntry::PassportId(PassportId(super::Id::Numeric(
                            860033327
                        ))))
                    ),
                    (
                        PassportKey::ExpirationYear,
                        Some(PassportEntry::ExpirationYear(ExpirationYear(2020)))
                    ),
                    (
                        PassportKey::HairColour,
                        Some(PassportEntry::HairColour(Colour::RGB(255, 255, 253)))
                    ),
                    (
                        PassportKey::BirthYear,
                        Some(PassportEntry::BirthYear(BirthYear(1937)))
                    ),
                    (
                        PassportKey::IssueYear,
                        Some(PassportEntry::IssueYear(IssueYear(2017)))
                    ),
                    (
                        PassportKey::CountryId,
                        Some(PassportEntry::CountryId(CountryId(Id::Numeric(147))))
                    ),
                    (
                        PassportKey::Height,
                        Some(PassportEntry::Height(Height::Cm(183)))
                    )
                ]
            );
        }
        const SAMPLE_DATA: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648"#;
        #[test]
        fn test_small() {
            match super::get_passports(SAMPLE_DATA) {
                Err(err) => {
                    println!("{:?}", err);
                    assert!(false)
                }
                Ok((remainder, passports)) => {
                    let results = passports.iter().cloned().collect::<Vec<_>>();
                    assert_eq!(remainder, "");
                    //   println!("{:?}", passports.first());
                    assert_eq!(results.len(), 2);
                }
            };
        }
    }
}
