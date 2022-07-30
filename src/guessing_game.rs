use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    println!("input your guess, please");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Greater => println!("{}", "too big".yellow()),
            Ordering::Equal => {
                println!("{}", "just right".green());
                break;
            }
        }
    }
}
