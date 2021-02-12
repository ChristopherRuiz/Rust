use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Random number generator between range of 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Infinite loop to allow for multiple tries until correct randonly generated number is selected
    loop {
        println!("Please input your guess.");

        // Declaring a mutable variable and setting its type to a string
        let mut guess = String::new();

        // reading in user input and catching any error with expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converts user input guess of type Str to guess of type integer (u32) and we trim any whitespace before and after
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Match guess with random number generated and continue if incorrect number selected
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The Secret Number Was {}", secret_number);
}