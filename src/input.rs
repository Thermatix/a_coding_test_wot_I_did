pub fn get_commands() -> String {
    match check_for_file_name() {
        Some(file) => load_input(file),
        None => accept_user_input(),
    }
}

fn check_for_file_name() -> Option<String> {
    use std::env;

    env::args().collect::<Vec<String>>().into_iter().nth(1)
}

fn load_input(file: String) -> String {
    use std::fs;
    fs::read_to_string(file).expect("Unable to read file")
}

// This sort of works but to finish you
// have to do CTRL-C (which actually types the escape code) and then
// press enter to finish, this is becuase stdin().read_line() locks
// the input and without using an external crate I'm not sure how to
// make this work as intended
fn accept_user_input() -> String {
    use std::io::stdin;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut buffer = String::new();

    while running.load(Ordering::SeqCst) {
        stdin().read_line(&mut buffer);
    }
    buffer.trim().to_owned()
}
