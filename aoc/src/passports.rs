use tracing::{error, warn};

#[derive(Debug, Eq, PartialEq)]
pub struct BirthYear(i32);
#[derive(Debug, Eq, PartialEq)]
pub struct IssueYear(i32);
#[derive(Debug, Eq, PartialEq)]
pub struct ExpirationYear(i32);
#[derive(Debug, Eq, PartialEq)]
pub enum Height {
    Cm(i32),
    In(i32),
}
#[derive(Debug, Eq, PartialEq)]
pub struct HairColour(u8, u8, u8);
#[derive(Debug, Eq, PartialEq)]
pub struct EyeColour(String);
#[derive(Debug, Eq, PartialEq)]
pub struct PassportId(isize);
#[derive(Debug, Eq, PartialEq)]
pub struct CountryId(isize);

#[derive(Debug, Eq, PartialEq)]
pub enum PassportEntry {
    BirthYear(BirthYear),
    IssueYear(IssueYear),
    ExpirationYear(ExpirationYear),
    Height(Height),
    HairColour(HairColour),
    EyeColour(EyeColour),
    PassportId(PassportId),
    CountryId(CountryId),
}
#[derive(Debug, Eq, PartialEq, Hash)]
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
pub struct Passport(std::collections::HashMap<PassportKey, PassportEntry>);
impl Passport {
    pub fn passport_from_entries(
        entries: Vec<(PassportKey, PassportEntry)>,
    ) -> Result<Self, PassportError> {
        let mut entry_map: std::collections::HashMap<PassportKey, PassportEntry> =
            std::collections::HashMap::default();
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut he = false;
        let mut ha = false;
        let mut eye = false;
        let mut pid = false;
        for (key, entry) in entries {
            match key {
                PassportKey::BirthYear => byr = true,
                PassportKey::IssueYear => iyr = true,
                PassportKey::ExpirationYear => eyr = true,
                PassportKey::Height => he = true,
                PassportKey::HairColour => ha = true,
                PassportKey::EyeColour => eye = true,
                PassportKey::PassportId => pid = true,
                PassportKey::CountryId => (),
            }
            if let Some(previous) = entry_map.insert(key, entry) {
                error!("found: {:?}", previous);

                return Err(PassportError("failed"));
            }
        }
        if byr && iyr && eyr && he && ha && eye && pid {
            Ok(Self(entry_map))
        } else {
            Err(PassportError("failed"))
        }
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
        character::complete::alpha1,
        character::complete::{digit1, newline, space0, space1},
        combinator::{map_res, recognize},
        multi::{separated_list0, separated_list1},
        sequence::tuple,
        IResult,
    };
    use std::str::FromStr;

    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
    }

    fn get_hair_colour(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, _) = tag("hcl:#")(input)?;
        let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

        Ok((
            input,
            (
                PassportKey::HairColour,
                PassportEntry::HairColour(HairColour(red, green, blue)),
            ),
        ))
    }

    fn get_passport_id(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, pid)) =
            tuple((tag("pid:"), map_res(recognize(digit1), isize::from_str)))(input)?;
        Ok((
            input,
            (
                PassportKey::PassportId,
                PassportEntry::PassportId(PassportId(pid)),
            ),
        ))
    }
    fn get_birth_year(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, year)) =
            tuple((tag("byr:"), map_res(recognize(digit1), i32::from_str)))(input)?;
        Ok((
            input,
            (
                PassportKey::BirthYear,
                PassportEntry::BirthYear(BirthYear(year)),
            ),
        ))
    }
    fn get_issue_year(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, year)) =
            tuple((tag("iyr:"), map_res(recognize(digit1), i32::from_str)))(input)?;
        Ok((
            input,
            (
                PassportKey::IssueYear,
                PassportEntry::IssueYear(IssueYear(year)),
            ),
        ))
    }
    fn get_expiration_year(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, year)) =
            tuple((tag("eyr:"), map_res(recognize(digit1), i32::from_str)))(input)?;
        Ok((
            input,
            (
                PassportKey::ExpirationYear,
                PassportEntry::ExpirationYear(ExpirationYear(year)),
            ),
        ))
    }

    fn get_passport_separator(input: &str) -> IResult<&str, &str> {
        let (input, bob) = recognize(tuple((newline, space0, newline)))(input)?;
        Ok((input, bob))
    }

    fn get_height(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_hgt, height, unit)) = tuple((
            tag("hgt:"),
            map_res(recognize(digit1), i32::from_str),
            alt((tag("cm"), tag("in"))),
        ))(input)?;
        match unit {
            "cm" => Ok((
                input,
                (
                    PassportKey::Height,
                    PassportEntry::Height(Height::Cm(height)),
                ),
            )),
            "in" => Ok((
                input,
                (
                    PassportKey::Height,
                    PassportEntry::Height(Height::In(height)),
                ),
            )),
            _ => panic!("alt failed"),
        }
    }

    fn get_eye_colour(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, colour)) = tuple((tag("ecl:"), alpha1))(input)?;
        Ok((
            input,
            (
                PassportKey::EyeColour,
                PassportEntry::EyeColour(EyeColour(format!("{}", colour))),
            ),
        ))
    }

    fn get_country_id(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
        let (input, (_, cid)) =
            tuple((tag("cid:"), map_res(recognize(digit1), isize::from_str)))(input)?;
        Ok((
            input,
            (
                PassportKey::CountryId,
                PassportEntry::CountryId(CountryId(cid)),
            ),
        ))
    }
    fn get_passport_entry(input: &str) -> IResult<&str, (PassportKey, PassportEntry)> {
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
    ) -> IResult<&str, Vec<(super::PassportKey, super::PassportEntry)>> {
        separated_list1(get_field_separator, get_passport_entry)(input)
    }

    fn get_passport(input: &str) -> IResult<&str, super::Passport> {
        let (result_input, entries) = get_passport_entries(input)?;
        match Passport::passport_from_entries(entries) {
            Ok(passport) => Ok((result_input, passport)),
            Err(err) => {
                error!("{:?}", err);
                Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Satisfy,
                )))
            }
        }
    }

    pub fn get_passports(input: &str) -> IResult<&str, Vec<super::Passport>> {
        separated_list0(get_passport_separator, get_passport)(input)
    }
    #[cfg(test)]
    mod tests {
        #[test]
        fn prop_ecl() {
            let (remainder, (pk, pv)) =
                super::get_passport_entry("ecl:gry").expect("parse should work");
            assert_eq!(remainder, "");
            assert_eq!(pk, super::PassportKey::EyeColour);
            assert_eq!(
                pv,
                super::PassportEntry::EyeColour(super::EyeColour(format!("gry")))
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
                super::PassportEntry::PassportId(super::PassportId(860033327))
            );
        }
        #[test]
        fn prop_hgt() {
            let (remainder, (pk, pv)) =
                super::get_passport_entry("hgt:183cm").expect("parse should work");
            assert_eq!(remainder, "");
            assert_eq!(pk, super::PassportKey::Height);
            assert_eq!(pv, super::PassportEntry::Height(super::Height::Cm(183)));
        }
        #[test]
        fn passport_entries() {
            let (reminder, passport) = super::get_passport_entries(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            )
            .expect("parse should work");
            assert_eq!(reminder, "");
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
                    assert_eq!(remainder, "");
                    assert_eq!(passports.len(), 2);
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Passport;

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
        let passports: Vec<Passport> = super::Passport::passports_from_str(SAMPLE_DATA);
        assert_eq!(passports.len(), 2);
    }
}
