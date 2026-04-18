use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number !");
        println!("Please input your guess.");

        //Using let to create a variable
        //mut is used to make the variable mutable
        //There's no mut that the variable is immutable
        //String::new() is used to create a new string
        let mut guess = String::new();

        //io::stdin() is used to get the input from the user
        //read_line() is used to read the input from the user
        //expect() is used to handle the error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line !");

        //The second guess is the value of the variable guess
        println!("You guess: {guess}");

        //Change data type from String to Interger
        let guess: u32 = guess.trim().parse().expect("Please type a number !");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("Very well !");
                break;
            }
        }
    }
}
