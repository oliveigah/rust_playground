mod benchmark;
mod branches;
mod functions;
mod guessing_game;
mod loops;
mod variables;

use std::io;

fn main() {
    println!("Which exercise to run?");
    println!("1 - Guessing Game");
    println!("2 - Variables");
    println!("3 - Functions");
    println!("4 - Branches");
    println!("5 - Loops");
    println!("6 - Benchmark");

    let mut excercise = String::new();

    io::stdin()
        .read_line(&mut excercise)
        .expect("failed to read game version");

    match excercise.as_str().trim() {
        "1" => guessing_game::start(),
        "2" => variables::start(),
        "3" => functions::start(),
        "4" => branches::start(),
        "5" => loops::start(),
        "6" => benchmark::start(),
        _ => println!("Invalid exercise!"),
    }
}
