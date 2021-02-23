use std::convert::From;

const DECIMAL_RADIX: u32 = 10;
const FIRST: usize = 0;
const SECOND: usize = 2;
const LAST: usize = 4;

#[derive(Debug)]
pub enum RoverCommand {
    GridSize { x: usize, y: usize },
    StartAt { x: usize, y: usize, direction: char },
    Move { actions: String },
}

// struct ErrorNotCommand(&'static str);

impl From<String> for RoverCommand {
    fn from(line: String) -> Self {
        let chars = line.chars().collect::<Vec<char>>();
        let is_digit = chars.iter().nth(FIRST).unwrap().is_digit(DECIMAL_RADIX);

        if is_digit {
            if chars.len() < LAST {
                Self::GridSize {
                    x: chars
                        .iter()
                        .nth(FIRST)
                        .unwrap()
                        .to_digit(DECIMAL_RADIX)
                        .unwrap() as usize,
                    y: chars
                        .iter()
                        .nth(SECOND)
                        .unwrap()
                        .to_digit(DECIMAL_RADIX)
                        .unwrap() as usize,
                }
            } else {
                Self::StartAt {
                    x: chars
                        .iter()
                        .nth(FIRST)
                        .unwrap()
                        .to_digit(DECIMAL_RADIX)
                        .unwrap() as usize,
                    y: chars
                        .iter()
                        .nth(SECOND)
                        .unwrap()
                        .to_digit(DECIMAL_RADIX)
                        .unwrap() as usize,
                    direction: chars.into_iter().nth(LAST).unwrap(),
                }
            }
        } else {
            Self::Move { actions: line }
        }
    }
}
