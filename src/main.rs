//! # Number Guessing Game
//! 
//! This is a simple command-line number guessing game implemented in Rust.
//! The program generates a random secret number between 1 and 100,
//! and the user is prompted to guess the number until they get it right.
//! 
//! ## Features
//! - Generates a random number using the `rand` crate.
//! - Accepts user input from the standard input (stdin).
//! - Disgard 
//! - Compares the guessed number with the secret number and provides feedback.
//! - Repeats until the user guesses correctly.
//! 
//! ## Usage
//! 1. Run the program.
//! 2. Enter a number between 1 and 100.
//! 3. Receive feedback on whether the guess is too low, too high, or correct.
//! 4. Repeat until you win.
//! 
//! ## Example Output
//! ```text
//! Guess the number!
//! Please input your guess.
//! 50
//! Too small
//! Please input your guess.
//! 75
//! Too big
//! Please input your guess.
//! 63
//! Winner
//! ```

use std::{cmp::Ordering, io}; // Input/Output Library STD: standard library. IO is a module
use rand::Rng; // Random library




fn main() {
    println!("Guess the number!"); // Print output with a newline
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Secret Number: {}",secret_number);  

    loop {
        println!("please input youe guess.");  // Print output on the same line

        let mut guess = String::new(); // Place to store user input. mut = mutable (able ot change). immutable (cant change) Default
        // '::' associate with the function
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // stdin- standard input for your terminal. read_line-handles the user input. Putting the input into 'guess'
        // the '&' symbol is a regference. Allows the variable to be accessed in other placces wihtout copiny the data into memory mulitple times
        // .expect handles potential failures aka errors. If results = ok, return the value, if results = err crashes and display msg
        // use .expect to avoid warning during compiling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // match { } ok returns num whcih is the number value
        // ERR value = use '_' which is a catch all statement. Dont matter what the err value is. It will continue the code
        // results (enum) have two options ok or Err so the guess is gonna match the result

        // let guess: u32 = guess.trim().parse().expect("Please Type a number"); 
        // Shadowing: lets you change type (string to int)
        //.trim = Removes white space in String. Orignal 5\n removes th e\n and white space
        //.parse converts the type to another type the : after the guess annotate the variable type
        //u32 = unsigned 32 bit intgeger (no negative) small positive numbers
        //.expect added due to potential error. If user uses something other than numbers
        
        println!("You guessed: {}" ,guess);

        // guess and secret number are the same variable 'u32'
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Winner");
                break; 
            }
    }
    }

}
