use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number! ");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();          //created a mutable variable that is currently bound to a new, empty instance of a String

        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = guess.trim().parse().expect("Please enter a number!");
        println!("You guessed: {}", guess);


        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }

    println!("The secret number is: {}", secret_num);
}

