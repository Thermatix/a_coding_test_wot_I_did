#[cfg(tests)]
mod tests;

mod data;
mod input;
mod rover_commands;

fn main() {
    let input: String = input::get_commands();

    println!("{}", input);
}
