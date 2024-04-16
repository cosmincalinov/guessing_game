use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number!");
                continue;
            }
        };

        if guess > 100 || guess < 1 {
            println!("Please enter a valid number!");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
