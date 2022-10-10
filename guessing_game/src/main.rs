use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut lower_bound: u16 = 1;
    let mut upper_bound: u16 = 100;

    loop{
        // println!("Please input your guess.");

        // let mut guess = String::new();

        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");

        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue
        // };

        let guess: u16 = (upper_bound + lower_bound) / 2;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lower_bound = guess;
            },
            Ordering::Greater => {
                println!("Too big!");
                upper_bound = guess;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}