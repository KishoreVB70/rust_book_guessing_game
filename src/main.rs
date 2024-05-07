use std::io;
fn main() {
    let mut guess = String::new();
    println!("Welcome to the guessing game, enter a number from 1 to 10");

    io::stdin().read_line(&mut guess).expect("Failed to get the input");
    println!("You have guessed: {guess}");
}
