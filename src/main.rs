use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Start the game
    println!("Welcome to Guessing number Game!");

    // generate random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number: {secret_number}");

    loop {
        // Ask for a number
        println!("Please enter a number.");

        let mut guess = String::new(); // assign an empty string

        io::stdin() // take input
            .read_line(&mut guess) // assign the input value to the guess string
            .expect("Error reading input."); // return if error is encountered

        let guess: u32 = match guess.trim().parse() {
            // Parse the input string into a u32 number
            Ok(num) => num,
            Err(_) => continue, // Ignore errors and continue
        };

        println!("Your guess is {guess}"); // Print the guess number

        match guess.cmp(&secret_number) {
            // Compare secret number with guessed number
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Haha! You win!");
                break; // Break out of the loop
            }
        }
    }
}
