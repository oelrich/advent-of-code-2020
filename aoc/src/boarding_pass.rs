use tracing::{error, warn};

#[derive(Debug, Eq, PartialEq)]
pub enum ColumnDirection {
    Left,
    Right,
}

impl ColumnDirection {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(format!("{}", c)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RowDirection {
    Back,
    Front,
}

impl RowDirection {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'B' => Ok(Self::Back),
            'F' => Ok(Self::Front),
            _ => Err(format!("{}", c)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BoardingPass {
    row: Vec<RowDirection>,
    column: Vec<ColumnDirection>,
}
impl BoardingPass {
    fn row(&self) -> i32 {
        let mut min = 0;
        let mut max = 127;
        for direction in &self.row[0..6] {
            let range = max - min;
            let delta = (range + range % 2) / 2;
            match direction {
                RowDirection::Back => {
                    min += delta;
                }
                RowDirection::Front => {
                    max -= delta;
                }
            }
        }
        match self.row[6] {
            RowDirection::Back => max,
            RowDirection::Front => min,
        }
    }
    fn column(&self) -> i32 {
        let mut min = 0;
        let mut max = 7;
        for direction in &self.column[0..2] {
            let range = max - min;
            let delta = (range + range % 2) / 2;
            match direction {
                ColumnDirection::Right => {
                    min += delta;
                }
                ColumnDirection::Left => {
                    max -= delta;
                }
            }
        }
        match self.column[2] {
            ColumnDirection::Right => max,
            ColumnDirection::Left => min,
        }
    }
    pub fn seat(&self) -> (i32, i32, i32) {
        let row = self.row();
        let column = self.column();
        (row, column, row * 8 + column)
    }
}

pub fn boarding_passes_from_string(input: &str) -> Vec<BoardingPass> {
    match parse::get_boarding_passes(input) {
        Ok((remainder, passes)) => {
            if !remainder.is_empty() {
                warn!("{}", remainder);
            }
            passes
        }
        Err(err) => {
            error!("{}", err);
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PASSES: &str = "FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
    const RESULTS: [(i32, i32, i32); 4] = [(44, 5, 357), (70, 7, 567), (14, 7, 119), (102, 4, 820)];
    #[test]
    fn pass_values() {
        let passes = boarding_passes_from_string(PASSES);
        assert_eq!(passes.len(), 4);
        for (idx, pass) in passes.iter().enumerate() {
            assert_eq!(pass.seat(), RESULTS[idx]);
        }
    }
}

mod parse {
    use super::*;
    use nom::{
        character::complete::{newline, one_of},
        combinator::{map_res, recognize},
        multi::{count, separated_list0},
        IResult,
    };

    fn get_first_seven(input: &str) -> IResult<&str, Vec<RowDirection>> {
        count(map_res(one_of("BF"), RowDirection::from_char), 7)(input)
    }
    fn get_last_three(input: &str) -> IResult<&str, Vec<ColumnDirection>> {
        count(map_res(one_of("LR"), ColumnDirection::from_char), 3)(input)
    }
    fn get_boarding_pass(input: &str) -> IResult<&str, BoardingPass> {
        let (input, row) = get_first_seven(input)?;
        let (input, column) = get_last_three(input)?;
        Ok((input, BoardingPass { row, column }))
    }
    pub fn get_boarding_passes(input: &str) -> IResult<&str, Vec<BoardingPass>> {
        separated_list0(recognize(newline), get_boarding_pass)(input)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn boarding_passes() {
            let (remainder, passes) =
                get_boarding_passes("FBFBBFFRLR\nFFFBBBFRRR").expect("passes");
            assert!(remainder.is_empty());
            assert_eq!(passes.len(), 2);
        }
        #[test]
        fn direction() {
            let (remainder, pass) = get_boarding_pass("FBFBBFFRLR").expect("result");
            assert!(remainder.is_empty());
            assert_eq!(
                pass.row,
                vec![
                    RowDirection::Front,
                    RowDirection::Back,
                    RowDirection::Front,
                    RowDirection::Back,
                    RowDirection::Back,
                    RowDirection::Front,
                    RowDirection::Front,
                ]
            );
            assert_eq!(
                pass.column,
                vec![
                    ColumnDirection::Right,
                    ColumnDirection::Left,
                    ColumnDirection::Right,
                ]
            )
        }
    }
}
