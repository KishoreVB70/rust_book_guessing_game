use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("welcome to the guessing game, enter the upper limit of the number you want to guess");
    let mut upper_limit = String::new();
    io::stdin().read_line(&mut upper_limit).expect("Failed to obtain the input");
    let upper_limit: u32 = upper_limit.trim().parse().expect("enter a number");

    // Generate a random number
    let rand_num = rand::thread_rng().gen_range(1..=upper_limit);

    //Start the loop for the game
    loop {
        // Create empty string
        let mut guess = String::new();
        println!("Enter a number from 1 to {upper_limit}");

        //Get input from the user
        io::stdin().read_line(&mut guess).expect("Failed to obtain the input");

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
                println!("The generated number is indeed {rand_num}");
                break;
            }
        }
    } 
}
