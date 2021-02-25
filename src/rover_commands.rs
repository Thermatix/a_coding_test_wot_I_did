use super::data::{self, Grid};
use std::convert::From;

const DECIMAL_RADIX: u32 = 10;
const FIRST: usize = 0;
const SECOND: usize = 2;
const LAST: usize = 4;

#[derive(Debug, Clone)]
pub enum RoverCommand {
    GridSize { x: usize, y: usize },
    StartAt { x: usize, y: usize, direction: char },
    Move { actions: Vec<Action> },
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Action {
    Left = b'L',
    Right = b'R',
    Move = b'M',
}

impl From<char> for Action {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            'M' => Self::Move,
            _ => panic!("Unknown Type"),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action::from(s.chars().nth(0).unwrap()))
    }
}

impl Into<char> for Action {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Debug)]
pub enum Errors {
    UnableToExecute(RoverCommand, Option<data::Errors>),
    NoGridPresent,
}

type ExecutionResult = Result<Option<Grid>, Errors>;
impl RoverCommand {
    // Never orignally intended for the function to take ownership
    // but doing this way just works
    pub fn execute(&self, grid: Option<Grid>) -> ExecutionResult {
        match self {
            Self::GridSize { x, y } => Ok(Some(Grid::new(*x, *y))),
            Self::StartAt { x, y, direction: d } => match grid {
                Some(mut grid) => match grid.new_rover(*x, *y, *d) {
                    Ok(_) => Ok(Some(grid)),
                    Err(e) => Err(Errors::UnableToExecute(self.clone(), Some(e))),
                },
                None => Err(Errors::NoGridPresent),
            },
            Self::Move { actions } => {
                let mut errors: Vec<data::Errors> = Vec::new();

                match grid {
                    Some(mut grid) => {
                        for a in actions.into_iter() {
                            match a {
                                Action::Move => {
                                    match grid.move_current_rover() {
                                        Err(e) => errors.push(e),
                                        _ => (),
                                    };
                                }

                                Action::Left | Action::Right => {
                                    match grid.change_current_rover_direction(a) {
                                        Err(e) => errors.push(e),
                                        _ => (),
                                    };
                                }
                            };
                        }

                        match errors.into_iter().next() {
                            Some(e) => Err(Errors::UnableToExecute(self.clone(), Some(e))),
                            None => Ok(Some(grid)),
                        }
                    }
                    None => Err(Errors::NoGridPresent),
                }
            }
        }
    }
}

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
            Self::Move {
                actions: line.chars().map(|c| c.into()).collect::<Vec<Action>>(),
            }
        }
    }
}
