use std::fmt;

#[derive(Debug)]
pub struct Rover {
    pub x: usize,
    pub y: usize,
    pub direction: char,
}

#[derive(Debug)]
pub struct Grid {
    pub area: Vec<usize>, // starts at zero, number - 1 is rovers index
    pub height: usize,
    pub width: usize,
    pub rovers: Vec<Rover>, // represented in area as rovers index + 1
    pub current: usize,     // ID of the current rover Move commands will be executed against
}

#[derive(Debug)]
pub enum Errors {
    RoverAlreadyPresent,
    NonExistantDirection,
    OffGrid,
}

const ZERO: usize = 0;
pub const ALLOWED_DIRECTIONS: &'static str = "LR";

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.direction)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer: String = String::new();

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if self.area[self.xy_idx(&x, &y)] > 0 {
                    buffer.push_str("#");
                } else {
                    buffer.push_str("X");
                }
            }
            buffer.push_str("\n");
        }
        write!(f, "{}", buffer)
    }
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            area: vec![ZERO; x * y],
            height: y,
            width: x,
            rovers: Vec::new(),
            current: ZERO,
        }
    }

    pub fn new_rover(&mut self, x: usize, y: usize, direction: char) -> Result<(), Errors> {
        if self.not_within_bounds(&x, &y) {
            Err(Errors::OffGrid)
        } else if self.can_set_at_destination(&x, &y) {
            self.rovers.push(Rover {
                x: x.clone(),
                y: y.clone(),
                direction: direction,
            });

            self.set_rover_at(x, y, self.rovers.len());
            self.current = self.rovers.len();
            Ok(())
        } else {
            Err(Errors::RoverAlreadyPresent)
        }
    }

    pub fn change_current_rover_direction(&mut self, new_direction: char) -> Result<(), Errors> {
        if ALLOWED_DIRECTIONS.contains(new_direction) {
            let mut rover = self.rovers.get_mut(self.current - 1).unwrap();
            rover.direction = match &rover.direction {
                'N' => {
                    if new_direction == 'L' {
                        'W'
                    } else {
                        'E'
                    }
                }
                'E' => {
                    if new_direction == 'L' {
                        'N'
                    } else {
                        'S'
                    }
                }
                'S' => {
                    if new_direction == 'L' {
                        'E'
                    } else {
                        'W'
                    }
                }
                'W' => {
                    if new_direction == 'L' {
                        'S'
                    } else {
                        'N'
                    }
                }
                _ => {
                    return Err(Errors::NonExistantDirection);
                }
            };
            Ok(())
        } else {
            Err(Errors::NonExistantDirection)
        }
    }

    pub fn move_current_rover(&mut self) -> Result<(), Errors> {
        let mut rover = self.rovers.pop().unwrap();

        let (x, y) = match &rover.direction {
            'N' => (rover.x, rover.y + 1),
            'E' => (rover.x + 1, rover.y),
            'S' => (rover.x, rover.y - 1),
            'W' => (rover.x - 1, rover.y),
            _ => {
                self.rovers.push(rover);
                return Err(Errors::NonExistantDirection);
            }
        };

        if self.not_within_bounds(&x, &y) {
            self.rovers.push(rover);
            Err(Errors::OffGrid)
        } else if self.can_set_at_destination(&x, &y) {
            self.remove_rover_at(rover.x, rover.y);
            self.set_rover_at(x, y, self.current);
            rover.x = x;
            rover.y = y;
            self.rovers.push(rover);
            Ok(())
        } else {
            self.rovers.push(rover);
            Err(Errors::RoverAlreadyPresent)
        }
    }

    pub fn get_rover_at(&self, x: &usize, y: &usize) -> Option<&Rover> {
        match &self.area[self.xy_idx(&x, &y)] {
            0 => None,
            n => self.rovers.get(n - (1 as usize)),
        }
    }

    pub fn xy_idx(&self, x: &usize, y: &usize) -> usize {
        x + (y * self.width)
    }

    fn remove_rover_at(&mut self, x: usize, y: usize) {
        let idx = self.xy_idx(&x, &y);
        self.area[idx] = ZERO;
    }

    fn set_rover_at(&mut self, x: usize, y: usize, rover_id: usize) {
        let idx = self.xy_idx(&x, &y);
        self.area[idx] = rover_id;
    }

    fn not_within_bounds(&self, x: &usize, y: &usize) -> bool {
        self.xy_idx(&x, &y) >= self.area.len()
    }

    fn can_set_at_destination(&self, x: &usize, y: &usize) -> bool {
        match self.get_rover_at(&x, &y) {
            None => true,
            Some(_) => false,
        }
    }
}

fn bca(rv: &usize, vta: i32, ub: &usize) -> usize {
    let r = (*rv).checked_add(vta as usize).unwrap_or(*rv);
    if r < *ub {
        r
    } else {
        *rv
    }
}
