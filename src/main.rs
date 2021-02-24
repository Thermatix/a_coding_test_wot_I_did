#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// #[cfg(tests)] // for some reason this causes it not to load the test files...
mod tests;

mod data;
mod input;
mod rover_commands;

fn main() -> Result<(), rover_commands::Errors> {
    use rover_commands::RoverCommand;

    let inputed_commands: String = input::get_commands();

    let rover_commands: Vec<RoverCommand> = inputed_commands
        .lines()
        .map(|line| line.to_string().into())
        .collect::<Vec<RoverCommand>>();

    let rovers = rover_commands
        .into_iter() // I'm not sure how to best handle errors in the fold :(
        .fold(None, |grid, command| command.execute(grid).unwrap())
        .unwrap()
        .rovers;

    for rover in rovers.into_iter() {
        println!("{}", rover);
    }
    Ok(())
}
