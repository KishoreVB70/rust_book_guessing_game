use std::io;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    let rand_num = rand::thread_rng().gen_range(1..=10);

    println!("Welcome to the guessing game, enter a number from 1 to 10");

    io::stdin().read_line(&mut guess).expect("Failed to get the input");
    println!("generated number: {rand_num}");
    println!("You have guessed: {guess}");
}
