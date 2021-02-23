use crate::rover_commands::RoverCommand;
use std::matches;

#[test]
fn test_if_grid_size() {
    let string = "3 3".to_string();
    let converted: RoverCommand = string.into();
    assert!(matches!(converted, RoverCommand::GridSize { x: 3, y: 3 }));
}

#[test]
fn test_if_start_at() {
    let string = "0 2 N".to_string();
    let converted: RoverCommand = string.into();

    assert!(matches!(
        converted,
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
    let converted: RoverCommand = string.into();

    assert!(matches!(converted, RoverCommand::Move { actions: string }));
}
