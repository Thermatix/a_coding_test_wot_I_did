use crate::data::*;
use crate::rover_commands::Action;
use std::matches;

#[test]
fn test_grid_is_correct_size() {
    let max_x: usize = 3;
    let max_y: usize = 5;
    let grid: Grid = Grid::new(max_x.clone(), max_y.clone());

    assert_eq!(grid.area.len(), max_x * max_y);
}

#[test]
fn test_if_grid_can_add_rover() {
    let mut grid: Grid = Grid::new(2 as usize, 2 as usize);
    let x: usize = 0;
    let y: usize = 1;

    let converted_loc = grid.xy_idx(&x, &y);

    assert!(matches!(grid.new_rover(x, y, 'N'), Ok(())));

    assert_eq!(grid.area[converted_loc], grid.current);
}

#[test]
fn test_if_grid_cant_add_rover_to_same_place() {
    let mut grid: Grid = Grid::new(2 as usize, 2 as usize);
    let x: usize = 0;
    let y: usize = 1;

    assert!(matches!(grid.new_rover(x.clone(), y.clone(), 'N'), Ok(())));

    assert!(matches!(
        grid.new_rover(x, y, 'N'),
        Err(Errors::RoverAlreadyPresent)
    ));
}

#[test]
fn test_if_grid_cant_add_rover_to_out_of_bounds() {
    let mut grid: Grid = Grid::new(2 as usize, 2 as usize);
    let x: usize = 1;
    let y: usize = 1;

    assert!(matches!(grid.new_rover(x.clone(), y.clone(), 'N'), Ok(())));

    assert!(matches!(
        grid.new_rover(x + 1 as usize, y, 'N'),
        Err(Errors::OffGrid(x, y))
    ));

    assert!(matches!(
        grid.new_rover(x, y + 1 as usize, 'N'),
        Err(Errors::OffGrid(x, y))
    ));
}

#[test]
fn test_if_grid_can_move_rover() {
    let mut grid: Grid = Grid::new(2 as usize, 2 as usize);
    let y: usize = 0;
    let x: usize = 1;

    grid.new_rover(x.clone(), y.clone(), 'N');
    assert_eq!(grid.get_rover_at(&x, &y).unwrap().direction, 'N'.into());
    grid.move_current_rover();
    assert!(matches!(grid.get_rover_at(&x, &y), None));
    assert_eq!(
        grid.get_rover_at(&x, &(y + 1 as usize)).unwrap().direction,
        'N'.into()
    );
}

#[test]
fn test_if_grid_can_change_rover_direction() {
    let mut grid: Grid = Grid::new(5 as usize, 5 as usize);
    let mut x: usize = 0;
    let mut y: usize = 0;

    grid.new_rover(x.clone(), y.clone(), 'N');
    grid.move_current_rover();
    assert!(matches!(grid.get_rover_at(&x, &y), None));
    y = y + 1;
    assert_eq!(grid.get_rover_at(&x, &y).unwrap().direction, 'N'.into());
    grid.change_current_rover_direction(Action::from('R'));
    assert_eq!(grid.get_rover_at(&x, &y).unwrap().direction, 'E'.into());
    grid.move_current_rover();
    x = x + 1;
    assert_eq!(grid.get_rover_at(&x, &y).unwrap().direction, 'E'.into());
}
