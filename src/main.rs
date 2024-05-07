use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess = String::new();
    let rand_num = rand::thread_rng().gen_range(1..=10);

    println!("Welcome to the guessing game, enter a number from 1 to 10");

    io::stdin().read_line(&mut guess).expect("Failed to get the input");

    //Convert the string to a u32
    // trim to remove any whitespace in the user input(error prevention)
    let guess: u32 = guess.trim().parse().expect("Enter a number rather than a letter");

    // Compare guess and the random value
    match guess.cmp(&rand_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("generated number: {rand_num}");
    println!("You have guessed: {guess}");
}
