use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Which version you want to play?");
    println!("1 - Easy");
    println!("2 - Normal");
    println!("3 - Hard");

    let mut game_vsn = String::new();

    io::stdin()
        .read_line(&mut game_vsn)
        .expect("failed to read game version");

    match game_vsn.as_str().trim() {
        "1" => guessing_game(10),
        "2" => guessing_game(100),
        "3" => guessing_game(1000),
        _ => println!("Invalid dificulty!"),
    }
}

fn guessing_game(max_number: u32) {
    let secret_number = rand::thread_rng().gen_range(1..=max_number);

    let optimal_moves = optimal_moves_for_guessing(secret_number, max_number);

    println!("--- Guess the number (Min: 1, Max: {max_number}, Target moves: {optimal_moves}) ---");

    println!("Secret number is {secret_number}");

    let mut total_moves = 0;

    loop {
        println!("--- Round {} ---", total_moves + 1);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        total_moves = total_moves + 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("--- END ---");
                println!("You won in {total_moves} moves!");

                let max_score: i32 = 1000;

                let score = match total_moves as i32 - optimal_moves as i32 {
                    moves_diff if moves_diff > 5 => 0,
                    moves_diff => {
                        let bonus_score_per_move =
                            (max_score as f32 / optimal_moves as f32).ceil() as i32;
                        let bonus_score = (moves_diff - 1) * bonus_score_per_move;
                        std::cmp::min(max_score - bonus_score, max_score * 2)
                    }
                };

                println!(
                    "Your score is: {score}! ({}%) ",
                    (score as f32 / max_score as f32) * 100.0
                );
                break;
            }
        }

        if total_moves as f32 > optimal_moves as f32 * 1.5 {
            println!("--- END ---");
            println!("You lost in {total_moves} moves! ):");
            println!("You score is: 0");
            break;
        }
    }
}

fn optimal_moves_for_guessing(secret_number: u32, max_guess: u32) -> u32 {
    let mut min_guess = 1;
    let mut max_guess = max_guess;
    let mut moves = 0;
    loop {
        moves = moves + 1;
        let guess = (max_guess + min_guess) / 2;
        match guess.cmp(&secret_number) {
            Ordering::Less => min_guess = guess + 1,
            Ordering::Greater => max_guess = guess - 1,
            Ordering::Equal => return moves,
        }
    }
}
