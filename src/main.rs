use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number
    let rand_num = rand::thread_rng().gen_range(1..=10);

    //Start the loop for the game
    loop {
        // Create empty string
        let mut guess = String::new();
        println!("Welcome to the guessing game, enter a number from 1 to 10");

        //Get input from the user
        io::stdin().read_line(&mut guess).expect("Failed to get the input");

        //Convert the string to a u32
        // trim to remove any whitespace in the user input(error prevention)
        let guess: u32 = guess.trim().parse().expect("Enter a number rather than a letter");
    
        // Compare guess and the random value
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Guess higher"),
            Ordering::Greater => println!("Guess lower"),
            // If they are equal, break the loop
            Ordering::Equal => {
                println!("You win! Game over!");
                break;
            }
        }

        // Print both the values for user reference
        println!("generated number: {rand_num}");
        println!("You have guessed: {guess}");
    } 
}
