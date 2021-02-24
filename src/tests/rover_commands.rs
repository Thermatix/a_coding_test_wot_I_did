use std::matches;

use crate::data;
use crate::rover_commands::RoverCommand;

#[test]
fn test_if_grid_size() {
    let string = "3 3".to_string();
    let executable: RoverCommand = string.into();
    assert!(matches!(executable, RoverCommand::GridSize { x: 3, y: 3 }));
}

#[test]
fn test_if_start_at() {
    let string = "0 2 N".to_string();
    let executable: RoverCommand = string.into();

    assert!(matches!(
        executable,
        RoverCommand::StartAt {
            x: 0,
            y: 2,
            direction: 'N'
        }
    ));
}

#[test]
fn test_if_move() {
    let string = "MLMRM".to_string();
    let executable: RoverCommand = string.into();

    assert!(matches!(
        executable,
        RoverCommand::Move { actions: _string }
    ));
}

#[test]
fn test_if_grid_size_executes() {
    let string = "3 3".to_string();
    let executable: RoverCommand = string.into();
    let grid = executable.execute(None).unwrap().unwrap();

    assert_eq!(grid.area.len(), 3 * 3);
}

#[test]
fn test_if_start_at_executes() {
    let mut grid = data::Grid::new(5, 5);
    let string = "0 2 N".to_string();
    let executable: RoverCommand = string.into();

    assert_eq!(grid.rovers.len(), 0);
    grid = executable.execute(Some(grid)).unwrap().unwrap();
    assert_eq!(grid.rovers.len(), 1);
}

#[test]
fn test_if_move_executes() {
    let mut grid = data::Grid::new(5, 5);
    let mut string = "1 2 N".to_string();
    let mut executable: RoverCommand = string.into();

    grid = executable.execute(Some(grid)).unwrap().unwrap();
    assert!(matches!(grid.get_rover_at(&1, &2), Some(..)));
    string = "MLMRM".to_string();
    executable = string.into();
    grid = executable.execute(Some(grid)).unwrap().unwrap();
    assert!(matches!(grid.get_rover_at(&0, &4), Some(..)));
}
