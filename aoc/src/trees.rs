use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Feature {
    Snow,
    Tree,
}
impl FromStr for Feature {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Tree),
            "." => Ok(Self::Snow),
            _ => Err(format!("unknown feature: {}", s)),
        }
    }
}

pub struct Map {
    width: usize,
    height: usize,
    features: std::collections::HashMap<(usize, usize), Feature>,
}
impl Map {
    pub fn feature(&self, x: usize, y: usize) -> Feature {
        let coordinate = (x % self.width, y % self.height);
        self.features
            .get(&coordinate)
            .cloned()
            .unwrap_or(Feature::Snow)
    }
    pub fn encounters(&self, x_delta: usize, y_delta: usize) -> usize {
        let mut count = 0;
        let mut next_x = x_delta;
        let mut next_y = y_delta;
        while next_y <= self.height {
            if self.feature(next_x, next_y) == Feature::Tree {
                count += 1;
            }
            next_x += x_delta;
            next_y += y_delta;
        }
        count
    }
}
impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut features = std::collections::HashMap::default();
        match parse::get_feature_lines(input) {
            Ok((remainder, lines)) => {
                if remainder.is_empty() {
                    let height = lines.len();
                    let width = if height != 0 {
                        lines.first().unwrap().len()
                    } else {
                        0
                    };
                    lines.iter().enumerate().for_each(|(y, fv)| {
                        fv.iter().enumerate().for_each(|(x, f)| match f {
                            Feature::Tree => {
                                if let Some(feature) = features.insert((x, y), Feature::Tree) {
                                    error!("Moved {:?} from ({},{})", feature, x, y)
                                }
                            }
                            Feature::Snow => (),
                        });
                    });
                    Ok(Self {
                        width,
                        height,
                        features,
                    })
                } else {
                    error!("{}", remainder);
                    Err(format!("input not empty after parsing: '{}'", remainder))
                }
            }
            Err(err) => {
                error!("{}", err);
                Err(format!("could not parse input"))
            }
        }
    }
}

mod parse {
    use std::str::FromStr;

    use nom::{
        character::complete::{anychar, newline},
        combinator::{map_res, recognize},
        multi::{many0, many1},
        sequence::tuple,
        IResult,
    };
    fn get_feature(input: &str) -> IResult<&str, super::Feature> {
        map_res(recognize(anychar), super::Feature::from_str)(input)
    }
    fn get_feature_line(input: &str) -> IResult<&str, Vec<super::Feature>> {
        let (input, (features, _)) = tuple((many1(get_feature), newline))(input)?;
        Ok((input, features))
    }
    pub fn get_feature_lines(input: &str) -> IResult<&str, Vec<Vec<super::Feature>>> {
        let (input, mut features) = many1(get_feature_line)(input)?;
        let (input, final_features) = many0(get_feature)(input)?;
        features.push(final_features);
        Ok((input, features))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    const SMALL_DATA: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
    #[test]
    fn test_small() {
        let map = super::Map::from_str(SMALL_DATA).expect("a map");
        assert_eq!(map.encounters(3, 1), 7);
    }
    #[test]
    fn test_bigger() {
        let map = super::Map::from_str(SMALL_DATA).expect("a map");
        assert_eq!(map.encounters(1, 1), 2);
        assert_eq!(map.encounters(3, 1), 7);
        assert_eq!(map.encounters(5, 1), 3);
        assert_eq!(map.encounters(7, 1), 4);
        assert_eq!(map.encounters(1, 2), 2);
    }
    #[test]
    fn test_bigger_more() {
        let map = super::Map::from_str(SMALL_DATA).expect("a map");
        let mut result = 1;
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .cloned()
            .for_each(|(x, y)| result *= map.encounters(x, y));
        assert_eq!(result, 336);
    }
    #[test]
    fn parse_small() {
        let map = super::Map::from_str(SMALL_DATA).expect("a map");
        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);
        assert_eq!(map.feature(0, 0), Feature::Snow);
        assert_eq!(map.feature(1, 0), Feature::Snow);
        assert_eq!(map.feature(2, 0), Feature::Tree);
        assert_eq!(map.feature(3, 0), Feature::Tree);

        assert_eq!(map.feature(0, 1), Feature::Tree);
        assert_eq!(map.feature(1, 1), Feature::Snow);
        assert_eq!(map.feature(2, 1), Feature::Snow);
        assert_eq!(map.feature(3, 1), Feature::Snow);
        assert_eq!(map.feature(4, 1), Feature::Tree);
        assert_eq!(map.feature(5, 1), Feature::Snow);
        assert_eq!(map.feature(6, 1), Feature::Snow);
        assert_eq!(map.feature(7, 1), Feature::Snow);
        assert_eq!(map.feature(8, 1), Feature::Tree);
        assert_eq!(map.feature(9, 1), Feature::Snow);
        assert_eq!(map.feature(10, 1), Feature::Snow);

        // Offsetting
        assert_eq!(map.feature(0 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(1 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(2 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(3 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(4 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(5 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(6 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(7 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(8 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(9 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(10 + 11, 1), Feature::Snow);
        // Offsetting
        assert_eq!(map.feature(0 + 11 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(1 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(2 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(3 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(4 + 11 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(5 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(6 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(7 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(8 + 11 + 11, 1), Feature::Tree);
        assert_eq!(map.feature(9 + 11 + 11, 1), Feature::Snow);
        assert_eq!(map.feature(10 + 11 + 11, 1), Feature::Snow);

        assert_eq!(map.feature(8, 10), Feature::Tree);
        assert_eq!(map.feature(9, 10), Feature::Snow);
        assert_eq!(map.feature(10, 10), Feature::Tree);
    }
}
